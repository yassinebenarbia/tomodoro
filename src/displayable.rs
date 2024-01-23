use tui::{widgets::StatefulWidget, layout::Rect};
use std::fmt::Debug;

use crate::state::State;

// pub trait Displayable{ 
//     fn highlight(&self);
//     fn width(&self) -> u16;
//     fn height(&self) -> u16;
//     fn x(&self) -> u16;
//     fn y(&self) -> u16;
// }

/// Each and every widget should compell to this interface 
pub trait Displayable: Debug + StatefulWidget<State = State> {
    fn width(&self) -> u16;
    fn height(&self) -> u16;
    fn x(&self) -> u16;
    fn y(&self) -> u16;
    /// this shall highlight the displayed widget
    fn highlight(&self);
    /// this is used to manage the state of the displayed object, and it should \
    /// be called at the end of the `render` call
    fn manage_state(&self, state: &mut State);

    fn layout(&self)->Rect;
}
