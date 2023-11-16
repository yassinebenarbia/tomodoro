use std::fmt::Debug;
use std::io;
use std::path::Display;
use tui::Terminal;
use tui::backend::CrosstermBackend;
use tui::buffer::{Cell, Buffer};
use tui::style::Color;
use tui::widgets::{StatefulWidget, BorderType};
use tui::{
    style::{Modifier, Style}, layout::Rect, widgets::{Block, Borders, Table},
};
use crate::State::State;
use crate::button_widget::ButtonWidget;
use crate::capabilities::compare_rect;
use crate::displayable::Displayable;

//TODO: add layout and style
//TODO: make a text struct that will held the text inside the button, if necessery
//TODO: add borders feature
//TODO: simplify the new method of the StatefullButton
//TODO: let the onclick and on hover closures take an instance of self as a parameter

///   A statefull button
///   frame: Rect,
///   layout: Rect,
///   widget: Block<'B>,
///   onhover: Option<Box<dyn FnMut(Rect, &mut Buffer, &mut ButtonState)>>,
///   onclick: Option<Box<dyn FnMut(Rect, &mut Buffer, &mut ButtonState)>>,
pub struct Button<'B> where {
    /// frame that constains the button
    frame: Rect,
    /// the area in which the button is displayed
    layout: Rect,
    /// how the button is displayed
    ///ToDo: change the name and the type, such that
    ///the new type implements the Widget trait, and can give access to
    ///it's style
    widget: ButtonWidget<'B>,
    /// onhover closure, will fier whenever the hovered state of the ButtonState state is true
    onhover: Option<Box<&'B mut dyn FnMut(Rect, &mut Buffer, &mut State)>>,
    /// onclick closure, will fier whenever the clicked state of the ButtonState state is true
    onclick: Option<Box<&'B mut dyn FnMut(Rect, &mut Buffer, &mut State)>>,
}

impl<'B> Debug for Button<'B> {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("frame")
            .field("frame", &self.frame)
            .field("layout", &self.layout)
            .field("widget", &self.widget)
            .field("onhover", &format!("onhover_closure"))
            .field("onclick", &format!("onclick_closure"))
            .finish()

    }
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
            layout: Rect::new(1, 1, 1, 1), 
            widget: ButtonWidget::default(),
            onhover: None,
            onclick: None,
        }

    }

}

impl<'B> StatefulWidget for Button<'B> {

    type State = State;

    fn render(self, area: Rect, buf: &mut tui::buffer::Buffer, state: &mut Self::State){

        let x_mid:u16 = ((area.x + area.width) as i16 / 2 as i16) as u16;
        let y_mid:u16 = ((area.y + area.height) as i16 / 2 as i16) as u16;

        let cell:Cell = Cell {
            symbol: ".".to_string(), fg: Color::Red, bg: Color::Cyan, modifier: Modifier::BOLD 
        };

        // buffer style
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

        let clicked = state.states.get("clicked").unwrap();

        if clicked.trim().parse::<bool>().unwrap() {

            match self.onclick {
                Some(mut func) =>{
                    func(area, buf, state);
                }
                None=>{}
            }

            state.states.insert("clicked".to_string(), "false".to_string());

        }

    }

}

impl<'B> Button<'B>{

    pub fn new<'b, F>(frame: Rect, layout: Rect, widget: ButtonWidget<'b>,
        onclick: Option<Box<&'b mut dyn FnMut(Rect, &mut Buffer, &mut State)>>,
        onhover: Option<Box<&'b mut dyn FnMut(Rect, &mut Buffer, &mut State)>>,
    )-> Button<'b>{

        match compare_rect(&layout, &frame){
            Ok(_)=>{
                Button{frame, layout, widget,  onclick, onhover}
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
        mut self, widget: ButtonWidget<'B>
    ) -> Button<'B>{
        self.widget = widget;
        self
    }

    /// under development
    pub fn style(mut self, widgetstyle: Style) -> Button<'B>{
        // self.style = widgetstyle;
        self
    }

    /// UnderDevelopment
    /// sets the text of the widget
    pub fn text(mut self, text: String) -> Button<'B> {self}

    pub fn onclick<T>(mut self, onclick: &'B mut T) -> Button<'B> where
        T: FnMut(Rect, &mut Buffer, &mut State)
    {
        self.onclick = Some(
            Box::new(onclick)
        );
        self
    }

    pub fn onhover<T>(mut self, onhover: &'B mut T) -> Button<'B> where
        T: FnMut(Rect, &mut Buffer, &mut State)
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

impl<'B> Displayable for  Button<'B>{

    fn manage_state(&self, state: &mut crate::State::State) {
        todo!()
    }

    fn x(&self) -> u16 {
        todo!()
    }

    fn y(&self) -> u16 {
        todo!()
    }

    fn width(&self) -> u16 {
        todo!()
    }
    fn height(&self) -> u16 {
        todo!()
    }
    fn highlight(&self) {
        todo!()
    }

}

#[derive(Debug, Default)]
pub struct ButtonState{
    hovered: bool,
    clicked: bool,
}

impl ButtonState {
    pub fn new(hovered: bool, clicked: bool) -> ButtonState {
        ButtonState{hovered, clicked}
    }
    pub fn clicked(& self) -> bool{
        self.clicked
    }
    pub fn hovered(& self) -> bool{
        self.hovered 
    }
    pub fn set_hover_state(&mut self, state: bool){
        self.hovered = state;
    }
    pub fn set_clicked_state(&mut self, state: bool){
        self.clicked = state;
    }
}
