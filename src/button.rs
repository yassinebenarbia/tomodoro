use std::io;
use tui::Terminal;
use tui::backend::CrosstermBackend;
use tui::style::Color;
use tui::widgets::BorderType;
use tui::{
    style::{Modifier, Style}, layout::Rect, widgets::{Block, Borders},
};
use crate::capabilities::compare_rect;

//ToDo: implement default
//ToDo: add layout and style
#[derive(Debug)]
pub struct Cadre<'B>{
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

impl<'B> Default for Cadre<'B> {

    fn default() -> Self {

        let backend = CrosstermBackend::new(io::stdout());
        let terminal = Terminal::new(backend).unwrap();

        Cadre{
            frame: terminal.size().unwrap(), 
            layout: Rect::new(0, 0, 20, 10), 
            widget: Block::default(),
        }

    }

}

impl<'B> Cadre<'B>{

    /// returns a Button instance
    pub fn new<'b>(frame: Rect, layout: Rect, widget: Block<'b>) -> Cadre<'b>{

        match compare_rect(&layout, &frame){
            Ok(_)=>{
                Cadre{ frame, layout, widget }
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
    ) -> Cadre<'B>{

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

    /// this represent the appearence of the widget
    pub fn widget(
        mut self, bg: Color, fg: Color, 
        modifier: Modifier, title: String,
        borders: Borders
    ) -> Cadre<'B>{
        self.widget = Block::default()
            .style(
                Style::default()
                    .fg(fg)
                    .bg(bg)
                    .add_modifier(modifier)
            )
            .title(title)
            .borders(borders)
            .border_type(BorderType::Rounded);
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
