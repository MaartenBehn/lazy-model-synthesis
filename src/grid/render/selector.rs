use octa_force::glam::{IVec2, Vec2};
use crate::grid::grid::Grid;
use crate::grid::identifier::GlobalPos;
use crate::grid::rules::ValueType;

const PIXELS_PER_NODE: f32 = 30.0;

pub struct Selector {
    pub last_selected: Option<IVec2>,
    pub value_type_to_place: Option<ValueType>,
}

impl Selector {
    pub fn new() -> Self {
        Selector {
            last_selected: None,
            value_type_to_place: None,
        }
    }

    pub fn add_to_render_data(&mut self, pos: Option<Vec2>, grid: &mut Grid) {
        if pos.is_none() {
            self.last_selected = None;
            return;
        }
        
        let node_pos = (pos.unwrap() / PIXELS_PER_NODE).as_ivec2();
        
        if node_pos.cmplt(IVec2::ZERO).any() || node_pos.cmpge(grid.chunk_size).any() {
            self.last_selected = None;
            return;
        }
        
        let chunk_node_index = grid
            .get_chunk_and_node_index_from_global_pos(GlobalPos(node_pos));

        grid
            .chunks[chunk_node_index.chunk_index]
            .render_data[chunk_node_index.node_index]
            .set_selector(true);

        self.last_selected = Some(node_pos);
    }
    
    pub fn clear_from_render_data(&mut self, grid: &mut Grid) {
        if let Some(last_pos) = self.last_selected {
            let chunk_node_index = grid
                .get_chunk_and_node_index_from_global_pos(GlobalPos(last_pos));

            grid.chunks[chunk_node_index.chunk_index]
                .render_data[chunk_node_index.node_index]
                .set_selector(false);
        }
    }
}