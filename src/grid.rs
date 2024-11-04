use octa_force::glam::{ivec2, IVec2};
use crate::render::node_render_data::NodeRenderData;
use crate::rules::ValueType;
use crate::visualization::GRID_SIZE;

pub type NodeIndex = usize;

const NODES_PER_GRID: usize = GRID_SIZE * GRID_SIZE;

#[derive(Copy, Clone)]
pub struct Grid {
    pub nodes: [Node; NODES_PER_GRID],
    pub render_data: [NodeRenderData; NODES_PER_GRID],
}

#[derive(Copy, Clone)]
pub struct Node{
    pub(crate) value: Option<ValueType>
}

#[derive(Copy, Clone)]
pub struct GridPosConverter {
    pub node_list_length: usize,
    pub grid_side_length: usize,
    pub grid_size: IVec2,
}

impl Grid {
    pub fn new(base_value: Option<ValueType>) -> Self {
        Grid {
            nodes: [Node::new(base_value); NODES_PER_GRID],
            render_data: [NodeRenderData::new(base_value); NODES_PER_GRID],
        }
    }
    
    pub fn set_node_value_with_index(&mut self, node_index: NodeIndex, value: Option<ValueType>) {
        self.nodes[node_index].value = value;
        
        if value.is_some() {
            self.render_data[node_index].set_selected_value_type(value.unwrap());
        } else {
            self.render_data[node_index].unselected_value_type();
        }
    } 
}

impl Node {
    pub fn new(base_value: Option<ValueType>) -> Self {
        Node {
            value: base_value,
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

