use std::io;
use tui::Terminal;
use tui::backend::CrosstermBackend;
use tui::style::Color;
use tui::{
    style::{Modifier, Style}, layout::Rect, widgets::{Block, Borders},
};

use crate::capabilities::compare_rect;

//ToDo: implement default
//ToDo: add layout and style
#[derive(Debug)]
pub struct Button<'B>{
    frame: Rect,
    layout: Rect,
    widget: Block<'B>
}
//  the Default implementation
//  -----------------
// | |----|         |
// | |    |         |
// | |----|         |
// |                |
// |                |
// |                |
// |                |
// |                |
// |                |
//  -----------------
// The big one is the frame and the smaller is the layout

impl<'B> Default for Button<'B> {

    fn default() -> Self {

        let backend = CrosstermBackend::new(io::stdout());
        let mut terminal = Terminal::new(backend).unwrap();

        Button{
            frame: terminal.size().unwrap(), 
            layout: Rect::new(0, 0, 20, 10), 
            widget: Block::default(),
        }

    }

}

impl<'B> Button<'B>{

    /// returns a Button instance
    pub fn new<'b>(frame: Rect, layout: Rect, widget: Block<'b>) -> Button<'b>{

        match compare_rect(&layout, &frame){
            Ok(_)=>{
                Button{ frame, layout, widget }
            },
            Err(msg)=>{
                panic!("following erro occured with widget {:?}\n{}", layout, msg)
            }
        }
    }

    /// this represent the shape of the button
    pub fn layout(
        mut self, x: u16, y: u16,
        width: u16, height: u16,
    ) -> Button<'B>{

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

    /// this represent the appearence of the widget
    pub fn widget(
        mut self, bg: Color, fg: Color, 
        modifier: Modifier, title: String,
        border: Vec<Borders>
    ) -> Button<'B>{
        self.widget = Block::default()
            .style(
                Style::default()
                    .fg(fg)
                    .bg(bg)
                    .add_modifier(modifier)
            )
            .title(title)
            .borders(border[0]);
        self
    }

    /// returns a clone of the button's widget
    pub fn get_widget(&mut self) ->Block<'B>{
        self.widget.clone()
    }
    /// returns a clone of the button's layout
    pub fn get_layout(&mut self)->Rect{
        self.layout.clone()
    }

}



