use tui::{widgets::StatefulWidget, layout::Rect};
use std::fmt::Debug;

use crate::state::State;

/// Each and every widget should compell to this interface 
pub trait Displayable: Debug + StatefulWidget<State = State> {
    /// this is used to manage the state of the displayed object, and it should \
    /// be called at the end of the `render` call
    fn manage_state(&self, state: &mut State);

    fn layout(&self)->Rect;
}
