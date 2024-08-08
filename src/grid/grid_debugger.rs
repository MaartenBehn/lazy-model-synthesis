use crate::grid::grid::Grid;
use crate::node_storage::NodeStorage;

pub struct GridDebugger{
    grids: Vec<Grid>,
    current: usize,
    length: usize,
}

impl GridDebugger {
    pub fn from_grid(grid: Grid, num_saved: usize) -> Self {
        GridDebugger{
            grids: vec![grid],
            current: 0,
            length: num_saved,
        }
    }

    pub fn tick_forward(&mut self, save_tick: bool) {
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

    pub fn tick_back(&mut self) {
        if self.current >= self.length -1 {
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
}