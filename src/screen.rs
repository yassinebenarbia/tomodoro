use std::fmt::Debug;
use tui::layout::Rect;
use crate::state::State;

#[derive(Debug, Default)]
pub struct StatetWrapper{
    pub wrapped: State,
}

/// compounds the state with the equivelent rectangle
#[derive(Debug)]
pub struct Compounder{
    pub states: Vec<(State, Rect)>
}

impl<'a> Compounder{
    pub fn new(states: Vec<(State , Rect)>) -> Compounder{
        Compounder { states }
    }

    pub fn encapsulate(inner: Vec<(State, Rect)>)->Compounder{
        let mut toreturn = vec![];
        for (state, rect) in inner.iter() {
            let temp_state = state.clone();
            toreturn.push((
                    temp_state,
                    rect.clone()
                    ))
        }
        Compounder::new(toreturn)
    }
}
