use std::time::Duration;
use octa_force::{App, BaseApp, egui, EngineConfig, EngineFeatureValue};
use octa_force::anyhow::Result;
use octa_force::egui::FontFamily::Proportional;
use octa_force::egui::{Align, FontId, Layout, Pos2};
use octa_force::egui::TextStyle::{Body, Button, Heading, Monospace, Small};
use octa_force::egui_winit::winit::event::WindowEvent;
use octa_force::glam::{uvec2, vec2};
use octa_force::gui::Gui;
use octa_force::vulkan::ash::vk::AttachmentLoadOp;
use crate::grid::debug_visulation::GridDebugVisulation;
use crate::grid::profile_visulation::GridProfileVisulation;


mod node;
mod value;
mod node_storage;
mod grid;
mod dispatcher;
mod history;
mod identifier;
mod util;
mod node_manager;

const WIDTH: u32 = 1920; // 2200;
const HEIGHT: u32 = 1080; // 1250;
const APP_NAME: &str = "Lazy Model Synthesis";

enum Visulation {
    None,
    GridDebug,
    GridProfile,
}

fn main() -> Result<()> {
    octa_force::run::<LazyModelSynthesis>(EngineConfig {
        name: APP_NAME.to_string(),
        start_size: uvec2(WIDTH, HEIGHT),
        ray_tracing: EngineFeatureValue::NotUsed,
        compute_rendering: EngineFeatureValue::Needed,
        validation_layers: EngineFeatureValue::Needed,
        shader_debug_printing: EngineFeatureValue::Needed,
    })
}

struct LazyModelSynthesis {
    grid_debug_renderer: GridDebugVisulation,
    grid_profile_renderer: GridProfileVisulation,
    current_renderer: Visulation,
    
    gui: Gui,
}

impl App for LazyModelSynthesis {
    fn new(base: &mut BaseApp<Self>) -> Result<Self> {

        let grid_debug_renderer = GridDebugVisulation::new(base)?;
        let grid_profile_renderer = GridProfileVisulation::new(base)?;
        
        let mut gui = Gui::new(
            &base.context,
            base.swapchain.format,
            base.swapchain.depth_format,
            &base.window,
            base.num_frames
        )?;

        Ok(Self {
            grid_debug_renderer,
            grid_profile_renderer,
            current_renderer: Visulation::None,
            gui,
        })
    }

    fn update(
        &mut self,
        base: &mut BaseApp<Self>,
        frame_index: usize,
        delta_time: Duration,
    ) -> Result<()> {
        if base.controls.f2 {
            self.current_renderer = Visulation::None;
        }
        
        match self.current_renderer {
            Visulation::None => {}
            Visulation::GridDebug => {
                self.grid_debug_renderer.update(base, frame_index, delta_time)?;
            }
            Visulation::GridProfile => {
                self.grid_profile_renderer.update(base, frame_index, delta_time)?;
            }
        }
        
        Ok(())
    }

    fn record_render_commands(
        &mut self,
        base: &mut BaseApp<Self>,
        frame_index: usize,
    ) -> Result<()> {
        match self.current_renderer {
            Visulation::None => { self.render_start_screen(base, frame_index)?; }
            Visulation::GridDebug => {
                self.grid_debug_renderer.record_render_commands(base, frame_index)?;
            }
            Visulation::GridProfile => {
                self.grid_profile_renderer.record_render_commands(base, frame_index)?
            }
        }

        Ok(())
    }
    
    fn on_window_event(&mut self, base: &mut BaseApp<Self>, event: &WindowEvent) -> Result<()> {
        match self.current_renderer {
            Visulation::None => {
                self.gui.handle_event(&base.window, event)
            }
            Visulation::GridDebug => {
                self.grid_debug_renderer.on_window_event(base, event)?;
            }
            Visulation::GridProfile => {
                self.grid_profile_renderer.on_window_event(base, event)?;
            }
        }
        
        

        Ok(())
    }

    fn on_recreate_swapchain(&mut self, _base: &mut BaseApp<Self>) -> Result<()> {
        Ok(())
    }
}

impl LazyModelSynthesis {
    fn render_start_screen(&mut self, base: &mut BaseApp<Self>, frame_index: usize,) -> Result<()> {
        let command_buffer = &base.command_buffers[frame_index];

        let size = base.swapchain.size;
        let swap_chain_image = &base.swapchain.images_and_views[frame_index].image;
        let swap_chain_view = &base.swapchain.images_and_views[frame_index].view;
        let swap_chain_depth_view = &base.swapchain.depht_images_and_views[frame_index].view;

        command_buffer.begin_rendering(swap_chain_view, swap_chain_depth_view, size, AttachmentLoadOp::CLEAR, None);

        self.gui.cmd_draw(command_buffer, size, frame_index, &base.window, &base.context, |ctx| {
            // Get current context style
            let mut style = (*ctx.style()).clone();

            // Redefine text_styles
            style.text_styles = [
                (Heading, FontId::new(60.0, Proportional)),
                (Body, FontId::new(15.0, Proportional)),
                (Monospace, FontId::new(14.0, Proportional)),
                (Button, FontId::new(40.0, Proportional)),
                (Small, FontId::new(10.0, Proportional)),
            ].into();

            // Mutate global style with above changes
            ctx.set_style(style);

            egui::CentralPanel::default().show(ctx, |ui| {
                ui.with_layout(Layout::top_down(Align::Center), |ui| {
                    
                    ui.add_space(ui.available_size().y  / 3.0);
                    
                    ui.heading("Lazy Model Synthesis");
                    
                    ui.add_space(50.0);
                    
                    
                    if ui.button("Grid (Debug Mode)").clicked() {
                        self.current_renderer = Visulation::GridDebug;
                    }

                    ui.add_space(20.0);

                    if ui.button("Grid (Profile Mode)").clicked() {
                        self.current_renderer = Visulation::GridProfile;
                    }
                })
            });

        })?;

        command_buffer.end_rendering();

        command_buffer.swapchain_image_render_barrier(swap_chain_image)?;
        
        Ok(())
    }
}
