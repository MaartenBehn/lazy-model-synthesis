use crate::grid::grid::Grid;
use crate::node_storage::NodeStorage;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum TickType {
    None,
    Back,
    Forward,
    ForwardSave,
}


pub struct GridDebugger{
    pub start_grid: Grid,
    pub grids: Vec<Grid>,
    pub current: usize,
    pub length: usize,

    pub next_tick: TickType,
}

impl GridDebugger {
    pub fn from_grid(grid: Grid, num_saved: usize) -> Self {
        GridDebugger{
            start_grid: grid.clone(),
            grids: vec![grid],
            current: 0,
            length: num_saved,
            next_tick: TickType::None,
        }
    }

    pub fn tick(&mut self) {
        match self.next_tick {
            TickType::None => {}
            TickType::Back => {self.tick_back()}
            TickType::Forward => {self.tick_forward(false)}
            TickType::ForwardSave => {self.tick_forward(true)}
        }
    }

    fn tick_forward(&mut self, save_tick: bool) {
        if self.current == 0 {
            if !save_tick {
                self.grids[0].tick();
                return;
            }

            let mut new_grid = self.grids[0].clone();
            new_grid.tick();
            self.grids.insert(0, new_grid);
            self.grids.truncate(self.length);
            return;
        }

        self.current -= 1;
    }

    fn tick_back(&mut self) {
        if self.current >= self.grids.len() - 1 {
            return;
        }

        self.current += 1;
    }

    pub fn get_grid(&self) -> &Grid {
        &self.grids[self.current]
    }

    pub fn get_grid_mut(&mut self) -> &mut Grid {
        &mut self.grids[self.current]
    }

    pub fn get_step_state(&self) -> (usize, usize) {
        (self.current, self.length)
    }

    pub fn reset(&mut self) {
        self.grids = vec![self.start_grid.clone()];
        self.current = 0;
    }
}