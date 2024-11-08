use octa_force::glam::{ivec2, IVec2};
use crate::value::Value;
use crate::visualization::GRID_SIZE;

pub type NodeIndex = usize;

pub const NODES_PER_GRID: usize = GRID_SIZE * GRID_SIZE;

#[derive(Copy, Clone)]
pub struct Grid {
    pub nodes: [Value; NODES_PER_GRID],
}

impl Grid {
    pub fn new(base_value: Value) -> Self {
        Grid {
            nodes: [base_value; NODES_PER_GRID],
        }
    }
}

pub fn get_node_index_from_pos(pos: IVec2) -> NodeIndex {
    (pos.x * GRID_SIZE as i32 + pos.y) as NodeIndex
}

pub fn get_pos_in_chunk_from_node_index(index: NodeIndex) -> IVec2 {
    let x = index as i32 / GRID_SIZE as i32;
    let y = index as i32 % GRID_SIZE as i32;
    ivec2(x, y)
}

pub fn is_pos_in_grid(pos: IVec2) -> bool {
    pos.x >= 0 && pos.y >= 0 && pos.x < GRID_SIZE as i32 && pos.y < GRID_SIZE as i32
}

