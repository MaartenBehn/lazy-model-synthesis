use octa_force::puffin_egui::puffin;

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
    fn tick_state(&mut self) -> bool;
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
        puffin::profile_function!();
        
        match self.next_tick {
            TickType::None => {}
            TickType::Back => {self.tick_back()}
            TickType::Forward => {self.tick_forward(false)}
            TickType::ForwardSave => {self.tick_forward(true)}
        }
    }

    fn tick_forward(&mut self, save_tick: bool) {
        puffin::profile_function!();
        
        if self.current == 0 {
            let mut new_state;
            {
                puffin::profile_scope!("Clone state");
                new_state = self.states[0].clone();
            }
            
            let changed = new_state.tick_state();
            
            if !changed || !save_tick {
                self.states[0] = new_state;
            } else {
                puffin::profile_scope!("Update state list");
                self.states.insert(0, new_state);
                self.states.truncate(self.length);
            }
            
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