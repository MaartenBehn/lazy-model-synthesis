use std::time::Duration;
use octa_force::gui::Gui;
use octa_force::anyhow::*;
use octa_force::{BaseApp, egui, glam};
use octa_force::egui::{Align, FontId, Frame, Id, Layout, Ui};
use octa_force::egui::FontFamily::Proportional;
use octa_force::egui::panel::Side;
use octa_force::egui::TextStyle::{Body, Button, Heading, Monospace, Small};
use octa_force::egui_winit::winit::event::WindowEvent;
use octa_force::glam::{IVec2, vec2};
use octa_force::vulkan::ash::vk::AttachmentLoadOp;
use crate::grid::grid::{Grid, ValueData};
use crate::dispatcher::random_dispatcher::RandomDispatcher;
use crate::general_data_structure::node::NodeT;
use crate::general_data_structure::value::ValueT;
use crate::go_back_in_time::node::GoBackNode;
use crate::go_back_in_time::node_manager::GoBackNodeManager;
use crate::go_back_in_time::value::GoBackValue;
use crate::grid::rules::{get_example_rules, NUM_REQS, NUM_VALUES, ValueType};
use crate::grid::identifier::{ChunkNodeIndex, GlobalPos, PackedChunkNodeIndex};
use crate::grid::render::renderer::GridRenderer;
use crate::LazyModelSynthesis;

const CHUNK_SIZE: usize = 1024;

pub struct GridProfileGoBackVisulation {
    pub gui: Gui,
    
    pub node_manager: GoBackNodeManager<
        Grid<GoBackNode<ValueData>, GoBackValue<ValueData>>,
        RandomDispatcher<ChunkNodeIndex, ValueData>,
        GlobalPos,
        ChunkNodeIndex,
        PackedChunkNodeIndex,
        ValueData,
        false,
    >,

    pub grid_renderer: GridRenderer,

    run: bool,
    run_ticks_per_frame: usize,
}

impl GridProfileGoBackVisulation {
    pub fn new(base: &mut BaseApp<LazyModelSynthesis>) -> Result<Self> {

        let mut gui = Gui::new(
            &base.context,
            base.swapchain.format,
            base.swapchain.depth_format,
            &base.window,
            base.num_frames
        )?;

        let grid_renderer = GridRenderer::new(&mut base.context, &mut gui.renderer, base.num_frames, CHUNK_SIZE, 1)?;

        Ok(GridProfileGoBackVisulation {
            node_manager: Self::create_node_manager(),
            gui,
            grid_renderer,
            run: false,
            run_ticks_per_frame: 50000,
        })
    }

    fn create_node_manager() -> GoBackNodeManager<
        Grid<GoBackNode<ValueData>, GoBackValue<ValueData>>,
        RandomDispatcher<ChunkNodeIndex, ValueData>,
        GlobalPos,
        ChunkNodeIndex,
        PackedChunkNodeIndex,
        ValueData,
        false,
    > {
        let mut grid = Grid::new(CHUNK_SIZE);
        grid.add_chunk(IVec2::ZERO);
        grid.rules = get_example_rules();

        let mut node_manager = GoBackNodeManager::new(grid.clone(), NUM_VALUES, NUM_REQS);
        node_manager.select_value(GlobalPos(IVec2::new(0, 0)), ValueData::new(ValueType::Stone));

        node_manager
    }

    pub fn update(
        &mut self,
        base: &mut BaseApp<LazyModelSynthesis>,
        frame_index: usize,
        _delta_time: Duration,
    ) -> Result<()> {
        if self.run {
            for _ in 0..self.run_ticks_per_frame {
                self.node_manager.tick();
            }

        }

        self.grid_renderer.set_chunk_data(0, CHUNK_SIZE, &self.node_manager.get_current().chunks[0].render_data);
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
            // Get current context style
            let mut style = (*ctx.style()).clone();

            // Redefine text_styles
            style.text_styles = [
                (Heading, FontId::new(20.0, Proportional)),
                (Body, FontId::new(15.0, Proportional)),
                (Monospace, FontId::new(14.0, Proportional)),
                (Button, FontId::new(15.0, Proportional)),
                (Small, FontId::new(10.0, Proportional)),
            ].into();

            // Mutate global style with above changes
            ctx.set_style(style);
            
            
            egui::SidePanel::new(Side::Left, Id::new("Side Panel")).show(ctx, |ui| {
                ui.with_layout(Layout::top_down(Align::LEFT), |ui| {
                    div(ui, |ui| {
                        ui.heading("Grid (Profile Mode)");
                    });

                    div(ui, |ui| {
                        ui.checkbox(&mut self.run, "run");
                        
                        ui.label("Ticks per frame: ");

                        ui.add(
                            egui::DragValue::new(&mut self.run_ticks_per_frame)
                                .speed(100)
                                .range(100..=100000),
                        );
                    });

                    
                    div(ui, |ui| {
                        if ui.button("clear").clicked() {
                            self.run = false;
                            self.node_manager = Self::create_node_manager();
                        }
                    });

                });
                
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

    pub fn on_window_event(&mut self, base: &mut BaseApp<LazyModelSynthesis>, event: &WindowEvent) -> Result<()> {
        self.gui.handle_event(&base.window, event);

        Ok(())
    }
}

fn egui_vec2_to_glam_vec2(v: egui::Vec2) -> glam::Vec2 {
    vec2(v.x, v.y)
}

fn div(ui: &mut Ui, add_contents: impl FnOnce(&mut Ui)) {
    Frame::none().show(ui, |ui| {
        ui.with_layout(Layout::left_to_right(Align::TOP), add_contents);
    });
}
