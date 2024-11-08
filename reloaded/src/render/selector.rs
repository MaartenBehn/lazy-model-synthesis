use octa_force::glam::{ivec2, IVec2, Vec2};
use crate::grid::{get_node_index_from_pos, Grid};
use crate::value::Value;
use crate::visualization::GRID_SIZE;

const PIXELS_PER_NODE: f32 = 30.0;

pub struct Selector {
    pub selected_pos: Option<IVec2>,
    pub value_type_to_place: Value,
}

impl Selector {
    pub fn new() -> Self {
        Selector {
            selected_pos: None,
            value_type_to_place: Value::default(),
        }
    }

    pub fn set_selected_pos(&mut self, pos: Option<Vec2>, grid: &mut Grid) {
        if pos.is_none() {
            self.selected_pos = None;
            return;
        }
        
        let node_pos = (pos.unwrap() / PIXELS_PER_NODE).as_ivec2();
        
        if node_pos.cmplt(IVec2::ZERO).any() || node_pos.cmpge(ivec2(GRID_SIZE as i32, GRID_SIZE as i32)).any() {
            self.selected_pos = None;
            return;
        }

        self.selected_pos = Some(node_pos);
    }
    
    pub fn clear_from_render_data(&mut self, grid: &mut Grid) {
        if let Some(last_pos) = self.selected_pos {
            let node_index = get_node_index_from_pos(last_pos);

            //grid.render_data[node_index].set_selector(false);
        }
    }
}