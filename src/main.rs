use std::time::Duration;
use octa_force::{App, BaseApp, EngineConfig, EngineFeatureValue};
use octa_force::anyhow::Result;
use octa_force::egui_winit::winit::event::WindowEvent;
use octa_force::glam::uvec2;
use octa_force::gui::Gui;
use crate::visualization::Visualization;

mod util;
pub mod render;
mod grid;
mod rules;
mod visualization;
mod grid_manager;

const WIDTH: u32 = 1920; // 2200;
const HEIGHT: u32 = 1080; // 1250;
const APP_NAME: &str = "Lazy Model Synthesis";

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
    visualization: Visualization,
    
    gui: Gui,
}

impl App for LazyModelSynthesis {
    fn new(base: &mut BaseApp<Self>) -> Result<Self> {

        let visualization = Visualization::new(base)?;
        
        let mut gui = Gui::new(
            &base.context,
            base.swapchain.format,
            base.swapchain.depth_format,
            &base.window,
            base.num_frames
        )?;

        Ok(Self {
            visualization,
            gui,
        })
    }

    fn update(
        &mut self,
        base: &mut BaseApp<Self>,
        frame_index: usize,
        delta_time: Duration,
    ) -> Result<()> {
        self.visualization.update(base, frame_index, delta_time)?;
        
        Ok(())
    }

    fn record_render_commands(
        &mut self,
        base: &mut BaseApp<Self>,
        frame_index: usize,
    ) -> Result<()> {
        self.visualization.record_render_commands(base, frame_index)?;

        Ok(())
    }
    
    fn on_window_event(&mut self, base: &mut BaseApp<Self>, event: &WindowEvent) -> Result<()> {
        self.visualization.on_window_event(base, event)?;

        Ok(())
    }

    fn on_recreate_swapchain(&mut self, _base: &mut BaseApp<Self>) -> Result<()> {
        Ok(())
    }
}
