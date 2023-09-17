use std::{io, fmt};
use tui::Terminal;
use tui::backend::CrosstermBackend;
use tui::buffer::{Cell, Buffer};
use tui::style::Color;
use tui::widgets::StatefulWidget;
use tui::{
    style::{Modifier, Style}, layout::Rect, widgets::{Block, Borders, Table},
};
use crate::capabilities::compare_rect;

//ToDo: implement default
//ToDo: add layout and style

///   A statefull button
///   frame: Rect,
///   layout: Rect,
///   widget: Block<'B>,
///   onhover: Option<Box<dyn FnMut(Rect, &mut Buffer, &mut ButtonState)>>,
///   onclick: Option<Box<dyn FnMut(Rect, &mut Buffer, &mut ButtonState)>>,
pub struct StatefullButton<'B> where {
    /// frame that constains the button
    frame: Rect,
    /// the area in which the button is displayed
    layout: Rect,
    /// how the button is displayed
    widget: Block<'B>,
    /// text to be displayed on the button, if anny
    text: Option<String>,
    /// onhover method, will fier whenever the hovered state of the ButtonState state is true
    onhover: Option<Box<&'B mut dyn FnMut(Rect, &mut Buffer, &mut ButtonState)>>,
    /// onclick method, will fier whenever the clicked state of the ButtonState state is true
    onclick: Option<Box<&'B mut dyn FnMut(Rect, &mut Buffer, &mut ButtonState)>>,
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

impl<'B> Default for StatefullButton<'B> {

    fn default() -> Self {

        let backend = CrosstermBackend::new(io::stdout());
        let mut terminal = Terminal::new(backend).unwrap();

        StatefullButton{
            frame: terminal.size().unwrap(), 
            layout: Rect::new(1, 1, 1, 1), 
            widget: Block::default(),
            text: None,
            onhover: None,
            onclick: None,
        }

    }

}

impl<'B> StatefulWidget for StatefullButton<'B> {

    type State = ButtonState;

    fn render(self, area: Rect, buf: &mut tui::buffer::Buffer, state: &mut Self::State){

        let x_mid:u16 = ((area.x + area.width) as i16 / 2 as i16) as u16;
        let y_mid:u16 = ((area.y + area.height) as i16 / 2 as i16) as u16;

        let cell:Cell = Cell {
            symbol: ".".to_string(), fg: Color::Red, bg: Color::Cyan, modifier: Modifier::BOLD 
        };

        if state.clicked {

            match self.onclick {
                Some(mut func) =>{
                    func(area, buf, state);
                }
                None=>{}
            }
            
            match self.onhover{
                Some(mut func) =>{
                    func(area, buf, state);
                }
                None=>{}
            }

            match self.text {
                Some(text)=>{
                    buf.set_string(x_mid, y_mid, text, 
                        Style::default()
                            .fg(Color::White)
                            .bg(Color::Black)
                            .add_modifier(Modifier::BOLD)
                    );
                }
                None=>{}
            }

            // let mut buffer = Buffer::filled(area, &cell);

            buf.set_style(area, Style::default()
                .fg(Color::Blue)
                .bg(Color::Green)
                .add_modifier(Modifier::BOLD)
            );
            
            state.clicked = false;
        }
    }
}

impl<'B> StatefullButton<'B>{

    //ToDo: change the closures implementation of the new method
    /// returns a StatefullButton instance
    pub fn new<'b, F>(frame: Rect, layout: Rect, widget: Block<'b>,
        onclick: Option<Box<&'b mut dyn FnMut(Rect, &mut Buffer, &mut ButtonState)>>,
        onhover: Option<Box<&'b mut dyn FnMut(Rect, &mut Buffer, &mut ButtonState)>>,
        text: Option<String>
    )-> StatefullButton<'b>{

        match compare_rect(&layout, &frame){
            Ok(_)=>{
                StatefullButton{frame, layout, widget, onclick, onhover, text}
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
    ) -> StatefullButton<'B>{

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
    ) -> StatefullButton<'B>{
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

    /// sets the text of the widget
    pub fn text(mut self, text: String) -> StatefullButton<'B> {
        self.text = Some(text);
        self

    }

    pub fn onclick<T>(mut self, onclick: &'B mut T) -> StatefullButton<'B> where
        T: FnMut(Rect, &mut Buffer, &mut ButtonState)
    {
        self.onclick = Some(
            Box::new(onclick)
        );
        self
    }

    pub fn onhover<T>(mut self, onhover: &'B mut T) -> StatefullButton<'B> where
        T: FnMut(Rect, &mut Buffer, &mut ButtonState)
    {
        self.onhover = Some(
            Box::new(onhover)
        );
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

#[derive(Debug)]
pub struct ButtonState{
    hovered: bool,
    clicked: bool,
}

impl ButtonState {
    pub fn new(hovered: bool, clicked: bool) -> ButtonState {
        ButtonState{hovered, clicked}
    }
    pub fn clicked(& self){
        self.clicked == true;
    }
    pub fn hovered(& self){
        self.hovered == true;
    }
    pub fn set_hover_state(&mut self, state: bool){
        self.hovered = state;
    }
    pub fn set_clicked_state(&mut self, state: bool){
        self.clicked = state;
    }
}
