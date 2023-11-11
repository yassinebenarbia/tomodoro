use std::collections::HashMap;


// desired behavior 
// let button_hash = HashMap::new();
// let button_state = State::new(button_has);
// let timer_hash = HashMap::new();
// let button_state = State::new(timer_hash);
/// This structure will hold the necessery states value for each\
/// and every widget
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
