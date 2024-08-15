use std::time::Duration;
use octa_force::{App, BaseApp, EngineConfig, EngineFeatureValue};
use octa_force::anyhow::*;
use octa_force::egui_winit::winit::event::WindowEvent;
use octa_force::glam::uvec2;
use crate::grid::visulation::GridVisulation;

mod node;
mod value;
mod node_storage;
mod grid;
mod dispatcher;
mod history;
mod identifier;
mod util;

const WIDTH: u32 = 1280; // 2200;
const HEIGHT: u32 = 720; // 1250;
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
    grid_renderer: GridVisulation,
}

impl App for LazyModelSynthesis {
    fn new(base: &mut BaseApp<Self>) -> Result<Self> {

        let grid_renderer = GridVisulation::new(base)?;

        Ok(Self {
            grid_renderer
        })
    }

    fn update(
        &mut self,
        base: &mut BaseApp<Self>,
        frame_index: usize,
        delta_time: Duration,
    ) -> Result<()> {

        self.grid_renderer.update(base, frame_index, delta_time)?;

        Ok(())
    }

    fn record_render_commands(
        &mut self,
        base: &mut BaseApp<Self>,
        frame_index: usize,
    ) -> Result<()> {

        self.grid_renderer.record_render_commands(base, frame_index)?;

        Ok(())
    }

    fn on_window_event(&mut self, base: &mut BaseApp<Self>, event: &WindowEvent) -> Result<()> {
        self.grid_renderer.on_window_event(base, event)?;

        Ok(())
    }

    fn on_recreate_swapchain(&mut self, _base: &mut BaseApp<Self>) -> Result<()> {



        Ok(())
    }
}
