use std::io;
use tui::Terminal;
use tui::backend::CrosstermBackend;
use tui::buffer::{Cell, Buffer};
use tui::style::Color;
use tui::widgets::{StatefulWidget, BorderType};
use tui::{
    style::{Modifier, Style}, layout::Rect, widgets::{Block, Borders, Table},
};
use crate::button_style::ButtonWidget;
use crate::capabilities::compare_rect;

//ToDo: add layout and style
//ToDo: make a text struct that will held the text inside the button, if necessery
//ToDo: add borders feature
//ToDo: simplify the new method of the StatefullButton

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
    ///ToDo: change the name and the type, such that
    ///the new type implements the Widget trait, and can give access to
    ///it's style
    widget: ButtonWidget<'B>,
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
            widget: ButtonWidget::default(),
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


        // println!("{:?}////{:?}", self.get_style(), self.get_layout());

        buf.set_style(
            self.get_layout(),
            self.get_widget().style
        );

        let symbols = BorderType::line_symbols(self.widget.border_type);

        //borders
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

        if self.widget.borders.contains(Borders::RIGHT | Borders::BOTTOM) {
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
            
            state.clicked = false;
        }

    }
}

impl<'B> StatefullButton<'B>{

    pub fn new<'b, F>(frame: Rect, layout: Rect, widget: ButtonWidget<'b>,
        onclick: Option<Box<&'b mut dyn FnMut(Rect, &mut Buffer, &mut ButtonState)>>,
        onhover: Option<Box<&'b mut dyn FnMut(Rect, &mut Buffer, &mut ButtonState)>>,
    )-> StatefullButton<'b>{

        match compare_rect(&layout, &frame){
            Ok(_)=>{
                StatefullButton{frame, layout, widget,  onclick, onhover}
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
        mut self, widget: ButtonWidget<'B>
    ) -> StatefullButton<'B>{
        self.widget = widget;
        self
    }

    /// under development
    pub fn style(mut self, widgetstyle: Style) -> StatefullButton<'B>{
        // self.style = widgetstyle;
        self
    }

    /// UnderDevelopment
    /// sets the text of the widget
    pub fn text(mut self, text: String) -> StatefullButton<'B> {self}

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
    pub fn get_widget(& self) ->ButtonWidget<'B>{
        self.widget.clone()
    }

    /// returns a clone of the button's layout
    pub fn get_layout(& self)->Rect{
        self.layout.clone()
    }

    // pub fn get_style(& self) ->Style{
    //     // Style::default()
    // }

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
