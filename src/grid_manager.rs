use std::collections::VecDeque;
use octa_force::glam::IVec2;
use crate::grid::{get_node_index_from_pos, is_pos_in_grid, Grid, NodeIndex};
use crate::rules::{get_example_rules, NeighborReq, Rule, ValueType};
use crate::util::state_saver::State;

#[derive(Clone)]
pub struct GridManager {
    pub grid: Grid,

    pub working_grids: VecDeque<WorkingGrid>,
    pub done_grids: Vec<Grid>,

    pub rules: Vec<Rule>,
    
}

#[derive(Clone)]
pub struct WorkingGrid {
    pub full_grid: Grid,
    pub empty_grid: Grid,
    orders: VecDeque<IVec2>
}

impl GridManager {
    pub fn new(grid: Grid) -> Self {
        GridManager{
            grid,
            working_grids: VecDeque::new(),
            done_grids: Vec::new(),
            rules: get_example_rules(),
        }
    }

    pub fn select_value(&mut self, pos: IVec2, value_type: ValueType) {
        self.working_grids.clear();
        
        let node_index = get_node_index_from_pos(pos);
        
        let mut working_grid: WorkingGrid = self.grid.to_owned().into();
        working_grid.set_node_value_with_node_index(node_index, Some(value_type));
        working_grid.orders.push_back(pos);
        
        self.working_grids.push_back(working_grid);
    }
    
    pub fn tick(&mut self) -> bool {
        
        let working_grid = self.working_grids.pop_front();
        if working_grid.is_none() {
            return false
        }
        
        let mut working_grid = working_grid.unwrap();
        let order = working_grid.orders.pop_front();
        if order.is_none() {
            return true
        }
        
        let done_working_grid = self.tick_order_on_working_grid(working_grid, order.unwrap());

        if done_working_grid.is_some() {
            self.done_grids.push(done_working_grid.unwrap().full_grid);
        }
        
        true
    }
    
    pub fn tick_order_on_working_grid(&mut self, mut working_grid: WorkingGrid, pos: IVec2) -> Option<WorkingGrid> {
        let node_index = get_node_index_from_pos(pos);
        let value_type = working_grid.get_node_value_with_node_index(node_index).unwrap();
        
        let mut grid_ok = true;
        
        let mut new_grids = vec![];
        for req in self.get_reqs_for_value_type(value_type) {
            let req_pos = pos + req.offset;
            
            if !is_pos_in_grid(req_pos) {
                continue
            }
            
            let req_node_index = get_node_index_from_pos(req_pos);
            
            let already_set_value = working_grid.empty_grid.nodes[req_node_index].value;
            if already_set_value.is_none() {
                for req_value_type in req.req_types.iter() {
                    let satisfied = working_grid.full_grid.nodes[req_node_index].value == Some(*req_value_type);
                    if satisfied {
                        continue
                    }

                    let mut new_working_grid = working_grid.to_owned();

                    new_working_grid.set_node_value_with_node_index(req_node_index, Some(*req_value_type));
                    new_working_grid.orders.push_back(req_pos);

                    new_grids.push(new_working_grid);
                }
            } else {
                let already_set_value = already_set_value.unwrap();
                let value_found = req.req_types.iter().find(|t| {**t == already_set_value}).is_some();
                
                if !value_found {
                    grid_ok = false;
                }
            }
        }
        
        if new_grids.is_empty() {
            if grid_ok {
                return Some(working_grid);
            }
        } else {
            for new_working_grid in new_grids {
                self.insert_working_grid(new_working_grid);
            }
        }

        None
    }
    
    pub fn get_reqs_for_value_type(&self, value_type: ValueType) -> &[NeighborReq] {
        &self.rules[value_type.get_value_nr() as usize].neighbor_reqs
    } 
    
    pub fn insert_working_grid(&mut self, working_grid: WorkingGrid) {
        let res = self.working_grids.binary_search_by(|w| {working_grid.orders.len().cmp(&w.orders.len())});
        let index = if res.is_err() { res.err().unwrap() } else { res.unwrap() };

        self.working_grids.insert(index, working_grid);
    }
    
}

impl From<Grid> for WorkingGrid {
    fn from(grid: Grid) -> Self {
        WorkingGrid {
            full_grid: grid,
            orders: VecDeque::new(),
            empty_grid: Grid::new(None),
        }
    }
}

impl WorkingGrid {
    pub fn set_node_value_with_node_index(&mut self, node_index: NodeIndex, value: Option<ValueType>) {
        self.full_grid.set_node_value_with_index(node_index, value);
        self.empty_grid.set_node_value_with_index(node_index, value);
    }

    pub fn get_node_value_with_node_index(&mut self, node_index: NodeIndex) -> Option<ValueType> {
        self.empty_grid.nodes[node_index].value
    }
}

impl State for GridManager {
    fn tick_state(&mut self) -> bool {
        self.tick() 
    }
}