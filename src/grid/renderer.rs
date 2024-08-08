use std::mem;
use std::mem::{align_of, size_of};
use octa_force::egui::{Image, TextureId};
use octa_force::egui_ash_renderer::Renderer;
use octa_force::glam::UVec2;
use octa_force::ImageAndView;
use octa_force::log::info;
use octa_force::puffin_egui::puffin;
use octa_force::vulkan::{Buffer, CommandBuffer, ComputePipeline, ComputePipelineCreateInfo, Context, DescriptorPool, DescriptorSet, DescriptorSetLayout, ImageBarrier, PipelineLayout, Sampler, WriteDescriptorSet, WriteDescriptorSetKind};
use octa_force::vulkan::ash::vk;
use octa_force::vulkan::ash::vk::{BufferUsageFlags, ComputePipelineCreateInfoBuilder, Format, ImageUsageFlags};
use octa_force::anyhow::Result;
use octa_force::egui::load::SizedTexture;
use octa_force::vulkan::gpu_allocator::MemoryLocation;
use crate::grid::grid::CHUNK_SIZE;
use crate::grid::node_render_data::NodeRenderData;
use crate::grid::visulation::GridVisulation;

const DISPATCH_GROUP_SIZE_X: u32 = 32;
const DISPATCH_GROUP_SIZE_Y: u32 = 32;

pub struct GridRenderer {
    pub wanted_size: UVec2,
    current_size: UVec2,

    descriptor_pool: DescriptorPool,
    egui_descriptor_layout: DescriptorSetLayout,
    egui_descriptor_sets: Vec<DescriptorSet>,
    sampler: Sampler,

    image_and_views: Vec<ImageAndView>,
    texture_ids: Vec<TextureId>,

    to_drop_image_data: Vec<(usize, Vec<ImageAndView>)>,

    render_descriptor_layout: DescriptorSetLayout,
    render_descriptor_sets: Vec<DescriptorSet>,
    render_pipeline_layout: PipelineLayout,
    render_pipeline: ComputePipeline,

    chunk_buffer: Buffer
}



impl GridRenderer {

    pub fn new(context: &mut Context, egui_renderer: &mut Renderer, num_frames: usize, loaded_chunks: usize) -> Result<Self> {

        let descriptor_pool = context.create_descriptor_pool(
            (num_frames * 2) as u32,
            &[
                vk::DescriptorPoolSize {
                    ty: vk::DescriptorType::COMBINED_IMAGE_SAMPLER,
                    descriptor_count: (num_frames * 2) as u32,
                },
                vk::DescriptorPoolSize {
                    ty: vk::DescriptorType::STORAGE_BUFFER,
                    descriptor_count: num_frames as u32,
                },
            ],
        ).unwrap();

        let egui_descriptor_layout = context.create_descriptor_set_layout(&[
            vk::DescriptorSetLayoutBinding {
                binding: 0,
                descriptor_count: 1,
                descriptor_type: vk::DescriptorType::COMBINED_IMAGE_SAMPLER,
                stage_flags: vk::ShaderStageFlags::FRAGMENT,
                ..Default::default()
            },
        ]).unwrap();

        let render_descriptor_layout = context.create_descriptor_set_layout(&[
            vk::DescriptorSetLayoutBinding {
                binding: 0,
                descriptor_count: 1,
                descriptor_type: vk::DescriptorType::STORAGE_IMAGE,
                stage_flags: vk::ShaderStageFlags::COMPUTE,
                ..Default::default()
            },
            vk::DescriptorSetLayoutBinding {
                binding: 1,
                descriptor_count: 1,
                descriptor_type: vk::DescriptorType::STORAGE_BUFFER,
                stage_flags: vk::ShaderStageFlags::COMPUTE,
                ..Default::default()
            },
        ]).unwrap();

        let sampler_info = vk::SamplerCreateInfo::builder();
        let sampler = context.create_sampler(&sampler_info)?;

        let mut egui_descriptor_sets = Vec::with_capacity(num_frames);
        let mut texture_ids = Vec::with_capacity(num_frames);

        let mut render_descriptor_sets = Vec::with_capacity(num_frames);

        for _ in 0..num_frames {
            let egui_descriptor_set = descriptor_pool.allocate_set(&egui_descriptor_layout).unwrap();

            let texture_id = egui_renderer.add_user_texture(egui_descriptor_set.inner, false);

            egui_descriptor_sets.push(egui_descriptor_set);
            texture_ids.push(texture_id);

            let render_descriptor_set = descriptor_pool.allocate_set(&render_descriptor_layout).unwrap();
            render_descriptor_sets.push(render_descriptor_set);
        }

        let render_pipeline_layout = context.create_pipeline_layout(
            &[&render_descriptor_layout],
            &[]
        )?;
        let render_pipeline = context.create_compute_pipeline(
            &render_pipeline_layout,
            ComputePipelineCreateInfo {
                shader_source: &include_bytes!("../../shaders/grid_render.comp.spv")[..],
            },
        )?;

        let chunk_buffer = context.create_buffer(
            BufferUsageFlags::STORAGE_BUFFER,
            MemoryLocation::CpuToGpu,
            (CHUNK_SIZE * CHUNK_SIZE * size_of::<NodeRenderData>()) as _
        )?;

        Ok(GridRenderer {
            wanted_size: UVec2::ZERO,
            current_size: UVec2::ZERO,

            descriptor_pool,
            egui_descriptor_layout,
            egui_descriptor_sets,
            texture_ids,
            sampler,

            image_and_views: vec![],
            to_drop_image_data: vec![],

            render_descriptor_layout,
            render_descriptor_sets,
            render_pipeline_layout,
            render_pipeline,

            chunk_buffer,
        })
    }

    pub fn update(&mut self, context: &mut Context, format: Format, image_index: usize) {
        puffin::profile_function!();

        for i in (0..self.to_drop_image_data.len()).rev() {
            let (drop_image_index, _) = &self.to_drop_image_data[i];
            if *drop_image_index != image_index {
               continue
            }

            self.to_drop_image_data.swap_remove(i);
        }

        let need_to_recreate = !self.image_and_views.is_empty() && self.current_size != self.wanted_size;

        let max_supported_size = context.physical_device.limits.max_image_dimension2_d;
        let wanted_size_ok = self.wanted_size.x > 0 && self.wanted_size.x < max_supported_size &&
            self.wanted_size.x > 0 && self.wanted_size.y < max_supported_size;

        if (need_to_recreate || self.image_and_views.is_empty()) && wanted_size_ok {
            info!("Creating Renderer {}x{}", self.wanted_size.x, self.wanted_size.y);

            let num_frames = self.egui_descriptor_sets.len();
            let mut image_and_views = Vec::with_capacity(num_frames);

            for i in 0..num_frames {

                let image = context.create_image(
                    ImageUsageFlags::SAMPLED | ImageUsageFlags::STORAGE,
                    MemoryLocation::GpuOnly,
                    format,
                    self.wanted_size.x,
                    self.wanted_size.y,
                ).unwrap();

                let view = image.create_image_view(false).unwrap();

                context.execute_one_time_commands(|cmd_buffer| {
                    cmd_buffer.pipeline_image_barriers(&[ImageBarrier {
                        image: &image,
                        old_layout: vk::ImageLayout::UNDEFINED,
                        new_layout: vk::ImageLayout::GENERAL,
                        src_access_mask: vk::AccessFlags2::NONE,
                        dst_access_mask: vk::AccessFlags2::NONE,
                        src_stage_mask: vk::PipelineStageFlags2::NONE,
                        dst_stage_mask: vk::PipelineStageFlags2::ALL_COMMANDS,
                    }]);
                }).unwrap();

                let image_and_view = ImageAndView { image, view };


                self.egui_descriptor_sets[i].update(&[
                    WriteDescriptorSet {
                        binding: 0,
                        kind: WriteDescriptorSetKind::CombinedImageSampler {
                            layout: vk::ImageLayout::GENERAL,
                            view: &image_and_view.view,
                            sampler: &self.sampler,
                        },
                    },
                ]);

                self.render_descriptor_sets[i].update(&[
                    WriteDescriptorSet {
                        binding: 0,
                        kind: WriteDescriptorSetKind::StorageImage {
                            layout: vk::ImageLayout::GENERAL,
                            view: &image_and_view.view,
                        },
                    },
                    WriteDescriptorSet {
                        binding: 1,
                        kind: WriteDescriptorSetKind::StorageBuffer {
                            buffer: &self.chunk_buffer,
                        },
                    },
                ]);

                image_and_views.push(image_and_view);
            }

            mem::swap(&mut self.image_and_views, &mut image_and_views);

            if need_to_recreate {
                self.to_drop_image_data.push((image_index, image_and_views))
            }

            self.current_size = self.wanted_size;
        }
    }

    pub fn set_chunk_data(&mut self, chunk_index: usize, chunk_data: &[NodeRenderData]) {
        self.chunk_buffer.copy_data_to_buffer_complex(chunk_data, chunk_index * CHUNK_SIZE * CHUNK_SIZE, align_of::<NodeRenderData>()).unwrap()
    }

    pub fn render(&mut self, command_buffer: &CommandBuffer, frame_index: usize) {
        if self.image_and_views.is_empty() {
            return;
        }

        command_buffer.bind_compute_pipeline(&self.render_pipeline);

        command_buffer.bind_descriptor_sets(
            vk::PipelineBindPoint::COMPUTE,
            &self.render_pipeline_layout,
            0,
            &[&self.render_descriptor_sets[frame_index]],
        );

        command_buffer.dispatch(
            (self.current_size.x / DISPATCH_GROUP_SIZE_X) + 1,
            (self.current_size.y / DISPATCH_GROUP_SIZE_Y) + 1,
            1,
        );
    }

    pub fn get_egui_image(&self, frame_index: usize) -> Option<Image> {
        if self.image_and_views.is_empty() {
            return None;
        }

        Some(Image::from_texture(SizedTexture::new(self.texture_ids[frame_index], self.current_size.as_vec2().as_ref())))
    }
}