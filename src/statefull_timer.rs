use std::{
    time::{Duration, self, SystemTime}, io::{self, Write}, fs, os::linux::raw::stat
};
use tui::{
    layout::Rect, backend::CrosstermBackend, Terminal, widgets::{StatefulWidget, BorderType, Borders}, text::{Spans, Span}, style::{Color, Style}
};

use crate::{timer_widget::TimerWidget, capabilities::{compare_rect, time_conversion}, timer_state::TimerState};

pub struct Timer{
    /// The Counting duration
    pub time: Duration,
    /// Frame
    pub frame: Rect,
    /// The area in which the timer is displayed
    pub layout: Rect,
    /// Timer widget, related to what is displayed and it's style
    /// Holds style, borders, border_style and border_type
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
        let terminal = Terminal::new(backend).unwrap();
        Timer { 
            time: Duration::from_secs(1500),
            layout: Rect::new(1, 1, 1, 1), 
            widget: TimerWidget::default(),
            frame: terminal.size().unwrap() 
        }
    }
}

impl StatefulWidget for Timer {
    type State = TimerState;
    fn render(self, area: Rect, buf: &mut tui::buffer::Buffer, state: &mut Self::State) {

        // checks the steate,
        // if the displayed time is eqal to the sate time
        // 1) substract time_step from the displayed time 
        // 2) render the widget
        // if the displayed time is not equal to zero
        // 1) substract time_step from the displayed time 
        // 2) render the widget

        if area.area() == 0 {
            return;
        }
        buf.set_style(area, self.widget.style);
        // TODO: inspect what do this line do
        let symbols = BorderType::line_symbols(self.widget.border_type);
        // TODO: inspect what inspect method do
        // sides
        if self.widget.borders.intersects(Borders::LEFT) {
            for y in area.top()..area.bottom() {
                buf.get_mut(area.left(), y)
                    .set_symbol(symbols.vertical)
                    .set_style(self.widget.border_style);
            }
        }
        if self.widget.borders.intersects(Borders::TOP) {
            for x in area.left()..area.right() {
                buf.get_mut(x, area.top())
                    .set_symbol(symbols.horizontal)
                    .set_style(self.widget.border_style);
            }
        }
        if self.widget.borders.intersects(Borders::RIGHT) {
            let x = area.right() - 1;
            for y in area.top()..area.bottom() {
                buf.get_mut(x, y)
                    .set_symbol(symbols.vertical)
                    .set_style(self.widget.border_style);
            }
        }
        if self.widget.borders.intersects(Borders::BOTTOM) {
            let y = area.bottom() - 1;
            for x in area.left()..area.right() {
                buf.get_mut(x, y)
                    .set_symbol(symbols.horizontal)
                    .set_style(self.widget.border_style);
            }
        }

        // corners
        if self.widget.borders.contains(Borders::RIGHT| Borders::BOTTOM) {
            buf.get_mut(area.right() - 1, area.bottom() - 1)
                .set_symbol(symbols.bottom_right)
                .set_style(self.widget.border_style);
        }
        if self.widget.borders.contains(Borders::RIGHT | Borders::TOP) {
            buf.get_mut(area.right() - 1, area.top())
                .set_symbol(symbols.top_right)
                .set_style(self.widget.border_style);
        }
        if self.widget.borders.contains(Borders::LEFT | Borders::BOTTOM) {
            buf.get_mut(area.left(), area.bottom() - 1)
                .set_symbol(symbols.bottom_left)
                .set_style(self.widget.border_style);
        }
        if self.widget.borders.contains(Borders::LEFT | Borders::TOP) {
            buf.get_mut(area.left(), area.top())
                .set_symbol(symbols.top_left)
                .set_style(self.widget.border_style);
        }

        //TODO: move this call out of the rendering loop
        let time = time_conversion(state.displayed);

        let time = Spans::from(vec![
            Span::styled(time, Style::default().fg(Color::Yellow))
        ]);

        // time
        let left_border_dx = if self.widget.borders.intersects(Borders::LEFT) {
            1
        } else {
            0
        };

        let right_border_dx = if self.widget.borders.intersects(Borders::RIGHT) {
            1
        } else {
            0
        };

        let time_area_width = area
            .width
            .saturating_sub(left_border_dx)
            .saturating_sub(right_border_dx);

        let time_dx = area.width.saturating_sub(time.width() as u16) / 2;
        let time_dy = area.height / 2;

        let time_x = area.left() + time_dx;
        let time_y = area.top() + time_dy;

        buf.set_spans(time_x, time_y, &time, time_area_width);

        state.manage_state();

    }

}

impl Timer {

    pub fn layout(mut self, x: u16, y: u16, width: u16, height: u16) -> Timer{
        let layout = Rect::new(x, y, width, height);
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
