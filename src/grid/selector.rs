use octa_force::glam::{IVec2, Vec2};
use crate::grid::grid_debugger::GridDebugger;

const PIXELS_PER_NODE: f32 = 30.0;

pub struct Selector {
    pub last_selected: Option<(IVec2, usize)>,
}

impl Selector {
    pub fn new() -> Self {
        Selector {
            last_selected: None,
        }
    }

    pub fn add_to_render_data(&mut self, pos: Option<Vec2>, grid_debugger: &mut GridDebugger) {
        if !grid_debugger.grids.is_empty() && pos.is_some() {
            let node_pos = (pos.unwrap() / PIXELS_PER_NODE).as_ivec2();
            let grid_index = grid_debugger.current;

            if node_pos.cmpge(IVec2::ZERO).all()
                && node_pos.cmplt(grid_debugger.grids[grid_index].chunk_size).all() {
                let (chunk_index, node_index) = grid_debugger
                    .grids[grid_index]
                    .get_chunk_and_node_index_from_global_pos(node_pos);

                grid_debugger
                    .grids[grid_index]
                    .chunks[chunk_index]
                    .render_data[node_index]
                    .set_selector(true);

                self.last_selected = Some((node_pos, grid_index));
            } else {
                self.last_selected = None;
            }
        } else {
            self.last_selected = None;
        }
    }
    
    pub fn clear_from_render_data(&mut self, grid_debugger: &mut GridDebugger) {
        if let Some((last_pos, last_grid_index)) = self.last_selected {
            let (chunk_index, node_index) = grid_debugger
                .grids[last_grid_index]
                .get_chunk_and_node_index_from_global_pos(last_pos);

            let grid_index = grid_debugger.current;
            grid_debugger
                .grids[grid_index]
                .chunks[chunk_index]
                .render_data[node_index]
                .set_selector(false);
        }
    }
}