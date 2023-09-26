use std::{time::{self, Duration}, io};
use tui::{
    layout::{Rect, self}, backend::CrosstermBackend, Terminal, widgets::{StatefulWidget, Block}
};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::{timer_widget::TimerWidget, capabilities::compare_rect, timer_state::TimerState};

pub struct Timer{
    /// the duration from which the timer should count
    pub time: Duration,
    /// frame
    pub frame: Rect,
    /// the area in which the timer is displayed
    pub layout: Rect,
    /// the widget of the timer, related to whatever displayed and style
    pub widget: TimerWidget
}
// ToDo: add the ability to change the style of 
// the displayed time
// Note: we can't specify the time step here, as it should 
// be controlled by on the loop, as we are not sure that 
// it will be renderd infinitely or just once


impl Default for Timer {
    fn default() -> Self {
        let backend = CrosstermBackend::new(io::stdout());
        let mut terminal = Terminal::new(backend).unwrap();
        Timer { 
            time: Duration::from_secs(1500),
            layout: Rect::new(1, 1, 1, 1), 
            widget: TimerWidget::default(),
            frame: terminal.size().unwrap() 
        }
    }
}

impl StatefulWidget for Timer{
    type State = TimerState;
    fn render(self, area: Rect, buf: &mut tui::buffer::Buffer, state: &mut Self::State) {
        // checks the steate,
        // if the displayed time is eqal to the sate time
        // 1) substract time_step from the displayed time 
        // 2) render the widget
        // if the displayed time is not equal to zero
        // 1) substract time_step from the displayed time 
        // 2) render the widget
    }
}

impl Timer {

    pub fn layout(mut self, x: u16, y: u16, width: u16, height: u16) -> Timer{
        let mut layout = Rect::new(x, y, width, height);
        match compare_rect(&self.frame, &layout){
            Ok(_)=>{
                self.layout = layout;
                self
            },
            Err(msg)=>{
                panic!("following erro occured with widget{:?}\n{}", layout, msg)
            }
        }
    }

    pub fn widget(mut self, widget: TimerWidget) -> Timer{
        self.widget = widget;
        self
    }

    pub fn time(mut self, time: Duration) -> Timer{
        self.time = time;
        self
    }

    pub fn get_widget(&mut self) -> TimerWidget{
        self.widget.clone()
    }

    pub fn get_timer(&mut self) -> Duration{
        self.time
    }

    pub fn get_layout(&mut self) -> Rect {
        self.layout.clone()
    }
}
