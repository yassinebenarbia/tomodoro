use tui::widgets::StatefulWidget;

/// Each and every widget should compell to this interface 
pub trait Displayable: StatefulWidget { 
    fn highlight(&self);
    fn width(&self) -> u16;
    fn height(&self) -> u16;
    fn x(&self) -> u16;
    fn y(&self) -> u16;
}

