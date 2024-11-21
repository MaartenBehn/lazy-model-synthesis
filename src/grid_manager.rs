use crate::value::VALUE_NONE;
use std::collections::VecDeque;
use octa_force::glam::IVec2;
use octa_force::log::debug;
use crate::grid::{get_node_index_from_pos, is_pos_in_grid, Grid, NodeIndex};
use crate::rules::{Rule, RuleReq};
use crate::util::state_saver::State;
use crate::value::{Value};

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
    pub orders: VecDeque<(IVec2, bool)>,
    pub satisfied_count: usize,
    pub set_count: usize,
}

impl GridManager {
    pub fn new(grid: Grid, rules: Vec<Rule>) -> Self {
        GridManager{
            grid,
            working_grids: VecDeque::new(),
            done_grids: Vec::new(),
            rules,
        }
    }

    pub fn select_value(&mut self, pos: IVec2, value: Value) {
        self.working_grids.clear();
        
        let node_index = get_node_index_from_pos(pos);
        
        let mut working_grid: WorkingGrid = self.grid.to_owned().into();
        working_grid.set_node_value_with_node_index(node_index, value, false);
        working_grid.orders.push_back((pos, false));
        working_grid.empty_grid.nodes[node_index].set_order(true);
        
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
        
        let (pos, satisfied) = order.unwrap();
        let done_working_grids = self.tick_order_on_working_grid(working_grid, pos, satisfied);

        if !done_working_grids.is_empty() {
            self.grid = done_working_grids[0].full_grid;
            self.working_grids.clear();
        }
        
        true
    }
    
    pub fn tick_order_on_working_grid(&mut self, mut working_grid: WorkingGrid, pos: IVec2, satisfied: bool) -> Vec<WorkingGrid> {
        let node_index = get_node_index_from_pos(pos);
        working_grid.empty_grid.nodes[node_index].set_order(false);
        
        if (satisfied) {
            debug!("Stop")
        }
        
        let value = working_grid.get_node_value_with_node_index(node_index);
        
        let mut new_grids = vec![];
        for rule_req in self.get_reqs_for_value(value) {

            let mut grid_ok = true;
            let mut added_value = false;
            let mut fully_satisfied = true;
            let mut new_working_grid = working_grid.to_owned();
            
            for (offset, req_value) in rule_req.reqs.iter() {
                let req_pos = pos + *offset;

                if !is_pos_in_grid(req_pos) {
                    continue
                }

                let req_node_index = get_node_index_from_pos(req_pos);
                let already_set_value = working_grid.empty_grid.nodes[req_node_index];

                if already_set_value.is_none() {
                    let req_satisfied = working_grid.full_grid.nodes[req_node_index].color_index == req_value.color_index;
                    new_working_grid.set_node_value_with_node_index(req_node_index, *req_value, req_satisfied);
                    
                    if !satisfied || !req_satisfied {
                        new_working_grid.orders.push_back((req_pos, req_satisfied));
                        new_working_grid.empty_grid.nodes[req_node_index].set_order(true);
                        fully_satisfied = false;
                    }

                    added_value = true;
                } else {
                    let already_set_value = already_set_value;
                    
                    if already_set_value.color_index != req_value.color_index {
                        grid_ok = false;
                    }
                }
            }
            
            if  grid_ok && (added_value || fully_satisfied) {
                new_grids.push(new_working_grid);
            }
        }

        let mut done_grids = vec![];
        for new_working_grid in new_grids {
            if new_working_grid.orders.is_empty() {
                done_grids.push(new_working_grid)
            } else {
                self.insert_working_grid(new_working_grid);
            }
        }

        done_grids
    }
    
    pub fn get_reqs_for_value(&self, value_type: Value) -> &[RuleReq] {
        &self.rules[value_type.get_value_nr() as usize].reqs
    } 
    
    pub fn insert_working_grid(&mut self, working_grid: WorkingGrid) {
        let res = self.working_grids.binary_search_by(|w| {w.get_score().cmp(&working_grid.get_score())});
        let index = if res.is_err() { res.err().unwrap() } else { res.unwrap() };

        self.working_grids.insert(index, working_grid);
    }
}

impl From<Grid> for WorkingGrid {
    fn from(grid: Grid) -> Self {
        WorkingGrid {
            full_grid: grid,
            orders: VecDeque::new(),
            satisfied_count: 0,
            empty_grid: Grid::new(VALUE_NONE),
            set_count: 0,
        }
    }
}

impl WorkingGrid {
    pub fn set_node_value_with_node_index(&mut self, node_index: NodeIndex, value: Value, satisfied: bool) {
        self.full_grid.nodes[node_index] = value;
        self.empty_grid.nodes[node_index] = value;
        self.set_count += 1;
        
        if satisfied {
            self.satisfied_count += 1;
        }
    }

    pub fn get_node_value_with_node_index(&mut self, node_index: NodeIndex) -> Value {
        self.empty_grid.nodes[node_index]
    }

    pub fn get_score(&self) -> i64 {
        //let not_s_orders = self.orders.iter().filter(|(_, s)| !s).count() as i64;
        
        //((1.0 - (self.satisfied_count as f32 / self.set_count as f32)) * 10.0) as i64 + not_s_orders

        self.set_count as i64 - self.satisfied_count as i64
    }
}

impl State for GridManager {
    fn tick_state(&mut self) -> bool {
        self.tick() 
    }
}