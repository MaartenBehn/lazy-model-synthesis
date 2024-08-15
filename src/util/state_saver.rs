use crate::history::History;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum TickType {
    None,
    Back,
    Forward,
    ForwardSave,
}

pub struct StateSaver<S> {
    start_state: S,
    states: Vec<S>,
    current: usize,
    length: usize,

    next_tick: TickType,
}

pub trait State: Clone {
    fn tick_state(&mut self);
}

impl<S: State> StateSaver<S> {
    pub fn from_state(history: S, num_saved: usize) -> Self {
        StateSaver {
            start_state: history.clone(),
            states: vec![history],
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
                self.states[0].tick_state();
                return;
            }

            let mut new_grid = self.states[0].clone();
            new_grid.tick_state();
            self.states.insert(0, new_grid);
            self.states.truncate(self.length);
            return;
        }

        self.current -= 1;
    }

    fn tick_back(&mut self) {
        if self.current >= self.states.len() - 1 {
            return;
        }

        self.current += 1;
    }

    pub fn get_state(&self) -> &S {
        &self.states[self.current]
    }

    pub fn get_state_mut(&mut self) -> &mut S {
        &mut self.states[self.current]
    }

    pub fn get_step_state(&self) -> (usize, usize) {
        (self.current, self.length)
    }

    pub fn reset(&mut self) {
        self.states = vec![self.start_state.clone()];
        self.current = 0;
    }
    
    pub fn set_next_tick(&mut self, next_tick: TickType) {
        self.next_tick = next_tick;
    }
}