use tui::widgets::StatefulWidget;
use std::fmt;

use crate::timer_state::TimerState;

/// Each and every widget should compell to this interface 
pub trait Displayable: StatefulWidget { 
    fn highlight(&self);
    fn width(&self) -> u16;
    fn height(&self) -> u16;
    fn x(&self) -> u16;
    fn y(&self) -> u16;
}
