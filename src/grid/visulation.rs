use std::fmt::format;
use std::time::Duration;
use octa_force::gui::Gui;
use octa_force::anyhow::*;
use octa_force::{BaseApp, egui, glam};
use octa_force::egui::{Align, Frame, Id, Layout, Ui, Widget};
use octa_force::egui::panel::Side;
use octa_force::egui_winit::winit::event::WindowEvent;
use octa_force::glam::{IVec2, vec2};
use octa_force::vulkan::ash::vk::AttachmentLoadOp;
use crate::grid::grid::{Grid, ValueData};
use crate::grid::grid_debugger::{GridDebugger, TickType};
use crate::grid::renderer::GridRenderer;
use crate::grid::rules::{get_example_rules, ValueType};
use crate::LazyModelSynthesis;
use crate::node_storage::NodeStorage;


pub struct GridVisulation {
    pub gui: Gui,

    pub grid_debugger: GridDebugger,
    pub grid_renderer: GridRenderer,

    run: bool,
}

impl GridVisulation {
    pub fn new(base: &mut BaseApp<LazyModelSynthesis>) -> Result<Self> {

        let mut grid = Grid::new();
        grid.add_chunk(IVec2::ZERO);
        grid.rules = get_example_rules();
        grid.add_initial_value(IVec2::new(0, 0), ValueData::new(ValueType::Stone));

        let mut gui = Gui::new(
            &base.context,
            base.swapchain.format,
            base.swapchain.depth_format,
            &base.window,
            base.num_frames
        )?;

        let grid_debugger = GridDebugger::from_grid(grid, 100);
        let grid_renderer = GridRenderer::new(&mut base.context, &mut gui.renderer, base.num_frames, 1)?;

        Ok(GridVisulation {
            grid_debugger,
            gui,
            grid_renderer,
            run: false,
        })
    }

    pub fn update(
        &mut self,
        base: &mut BaseApp<LazyModelSynthesis>,
        frame_index: usize,
        delta_time: Duration,
    ) -> Result<()> {
        if self.run {
            self.grid_debugger.set_next(TickType::ForwardSave);
        }


        self.grid_debugger.tick();
        self.grid_renderer.set_chunk_data(0, &self.grid_debugger.get_grid().chunks[0].render_data);
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
            egui::SidePanel::new(Side::Left, Id::new("Side Panel")).show(ctx, |ui| {
                ui.with_layout(Layout::top_down(Align::LEFT), |ui| {
                    div(ui, |ui| {
                        ui.label("Tick: ");

                        if ui.button("<<<").clicked() {
                            self.run = false;
                            self.grid_debugger.set_next(TickType::Back);
                        }

                        if ui.button(">>>").clicked() {
                            self.run = false;
                            self.grid_debugger.set_next(TickType::ForwardSave);
                        }
                    });

                    div(ui, |ui| {
                        ui.label("saved ticks:");
                    });

                    div(ui, |ui| {
                        let (current_saved, num_saved) = self.grid_debugger.get_step_state();
                        egui::ProgressBar::new(1.0 - (current_saved as f32 / num_saved as f32)).ui(ui);
                    });

                    div(ui, |ui| {
                        ui.checkbox(&mut self.run, "run");

                        if ui.button("clear").clicked() {
                            self.run = false;
                            self.grid_debugger.reset()
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