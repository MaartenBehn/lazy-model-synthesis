use std::time::Duration;
use hot_lib_reloader::LibReloadObserver;
use octa_force::{Engine, OctaResult};
use octa_force::egui_winit::winit::event::WindowEvent;
use octa_force::log::debug;
#[cfg(feature = "reload")]
use hot_lib::*;
#[cfg(not(feature = "reload"))]
use reloaded::*;

// The value of `dylib = "..."` should be the library containing the hot-reloadable functions
// It should normally be the crate name of your sub-crate.
#[cfg(feature = "reload")]
#[hot_lib_reloader::hot_module(dylib = "reloaded")]
mod hot_lib {
    // Reads public no_mangle functions from lib.rs and  generates hot-reloadable
    // wrapper functions with the same signature inside this module.
    // Note that this path relative to the project root (or absolute)
    hot_functions_from_file!("reloaded/src/lib.rs");

    #[lib_updated]
    pub fn was_updated() -> bool {}

    // Because we generate functions with the exact same signatures,
    // we need to import types used
    use std::time::Duration;
    use crate::WindowEvent;
    use octa_force::EngineConfig;
    use octa_force::Engine;
    use octa_force::OctaResult;
    pub use reloaded::State;
}

fn main() {
    let config = new_engine_config();
    octa_force::run::<Game>(config).unwrap()
}

pub struct Game {
    state: State,
}

impl Game {
    fn check_recreate(&mut self, engine: &mut Engine) -> OctaResult<()> {
        #[cfg(feature = "reload")]
        {
            if was_updated() {
                debug!("Was hot reloaded");
                self.state = new_state(engine)?;
            }
        }
        
        Ok(())
    }
}

impl octa_force::State for Game {
    fn new(engine: &mut Engine) -> OctaResult<Self> {
        Ok(Self {
            state: new_state(engine)?,
        })
    }
    
    fn update(&mut self, engine: &mut Engine, frame_index: usize, delta_time: Duration) -> OctaResult<()> {
        update(&mut self.state, engine, frame_index, delta_time)
    }

    fn record_render_commands(
        &mut self,
        engine: &mut Engine,
        frame_index: usize,
    ) -> OctaResult<()> {
        record_render_commands(&mut self.state, engine, frame_index)
    }

    fn on_window_event(&mut self, engine: &mut Engine, event: &WindowEvent) -> OctaResult<()> {
        on_window_event(&mut self.state, engine, event)
    }
    
    fn on_recreate_swapchain(&mut self, engine: &mut Engine) -> OctaResult<()> {
        on_recreate_swapchain(&mut self.state, engine)
    }
}