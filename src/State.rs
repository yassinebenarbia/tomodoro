use std::collections::HashMap;

use crate::directions::Commands;

// desired behavior 
// let button_hash = HashMap::new();
// let button_state = State::new(button_has);
// let timer_hash = HashMap::new();
// let button_state = State::new(timer_hash);
/// This structure will hold the necessery states value for each\
/// and every widget
///
/// cycles: string, counter of the current cyle
/// max_cycles: u32, the number of the maximum allowed cycles
/// working: boolean, is the timer working or not
/// prev_diff: u32, holds the difference between the previous\
/// and current frame in seconds
/// start: u64(seconds), the systime since the application started
/// duration: u32(seconds), the length of the cycle
/// displayed: u32(seconds), this holds the displayed time buffer
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct State{
    pub states: HashMap<String, String>
}


impl State{

    pub fn get_states(&mut self) -> &HashMap<String, String>{
        &self.states
    }

    pub fn new(states: HashMap<String, String>) -> Self{
        Self { states }
    }

}
