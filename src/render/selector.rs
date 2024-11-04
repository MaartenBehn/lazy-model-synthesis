use octa_force::glam::{ivec2, IVec2, Vec2};
use crate::grid::{get_node_index_from_pos, Grid, GridPosConverter};
use crate::rules::ValueType;
use crate::visualization::GRID_SIZE;

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
        
        if node_pos.cmplt(IVec2::ZERO).any() || node_pos.cmpge(ivec2(GRID_SIZE as i32, GRID_SIZE as i32)).any() {
            self.last_selected = None;
            return;
        }
        
        let node_index = get_node_index_from_pos(node_pos);
        grid.render_data[node_index].set_selector(true);

        self.last_selected = Some(node_pos);
    }
    
    pub fn clear_from_render_data(&mut self, grid: &mut Grid) {
        if let Some(last_pos) = self.last_selected {
            let node_index = get_node_index_from_pos(last_pos);

            grid.render_data[node_index].set_selector(false);
        }
    }
}