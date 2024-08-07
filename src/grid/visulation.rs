use std::time::Duration;
use octa_force::gui::Gui;
use octa_force::anyhow::*;
use octa_force::{BaseApp, egui, glam};
use octa_force::egui::{Id, Image};
use octa_force::egui::load::SizedTexture;
use octa_force::egui::panel::Side;
use octa_force::glam::{UVec2, vec2};
use octa_force::log::info;
use octa_force::vulkan::ash::vk::AttachmentLoadOp;
use crate::grid::Grid;
use crate::grid::renderer::GridRenderer;
use crate::LazyModelSynthesis;


pub struct GridVisulation {
    pub grid: Grid,
    pub gui: Gui,

    pub grid_renderer: GridRenderer,
}



impl GridVisulation {
    pub fn new(base: &mut BaseApp<LazyModelSynthesis>) -> Result<Self> {

        let grid = Grid::new();
        let mut gui = Gui::new(
            &base.context,
            base.swapchain.format,
            base.swapchain.depth_format,
            &base.window,
            base.num_frames
        )?;

        let grid_renderer = GridRenderer::new(&mut base.context, base.num_frames, &mut gui.renderer)?;

        Ok(GridVisulation {
            grid,
            gui,
            grid_renderer,
        })
    }

    pub fn update(
        &mut self,
        base: &mut BaseApp<LazyModelSynthesis>,
        frame_index: usize,
        delta_time: Duration,
    ) -> Result<()> {

        self.grid_renderer.update(&mut base.context, base.swapchain.format, frame_index);

        Ok(())
    }

    pub fn record_render_commands(
        &mut self,
        base: &mut BaseApp<LazyModelSynthesis>,
        frame_index: usize,
    ) -> Result<()> {

        let command_buffer = &base.command_buffers[frame_index];

        let size = base.swapchain.size;
        let swap_chain_image = &base.swapchain.images_and_views[frame_index].image;
        let swap_chain_view = &base.swapchain.images_and_views[frame_index].view;
        let swap_chain_depth_view = &base.swapchain.depht_images_and_views[frame_index].view;

        self.grid_renderer.render(command_buffer, frame_index);

        command_buffer.begin_rendering(swap_chain_view, swap_chain_depth_view, size, AttachmentLoadOp::CLEAR, None);

        self.gui.cmd_draw(command_buffer, size, frame_index, &base.window, &base.context, |ctx| {
            egui::SidePanel::new(Side::Left, Id::new("Side Panel")).show(ctx,|ui| {

            });


            egui::CentralPanel::default().show(ctx, |ui| {
                let available_size = egui_vec2_to_glam_vec2(ui.available_size());
                self.grid_renderer.wanted_size = available_size.as_uvec2();

                let image = self.grid_renderer.get_egui_image(frame_index);
                if image.is_some() {
                    ui.add(image.unwrap());
                }
            });
        })?;

        command_buffer.end_rendering();

        command_buffer.swapchain_image_render_barrier(swap_chain_image)?;

        Ok(())
    }
}

pub fn egui_vec2_to_glam_vec2(v: egui::Vec2) -> glam::Vec2 {
    vec2(v.x, v.y)
}