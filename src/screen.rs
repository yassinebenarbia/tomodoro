use std::{fmt::Debug, io::Stdout};
use tui::{widgets::StatefulWidget, layout::Rect, Terminal, backend::CrosstermBackend};

use crate::{displayable::Displayable, state::State, config::Config, widget_fixer::Fixer};

// question:
// does dealing with the widget inside the screen depend on the type of the widget, 
// meaning being a stateless and statefull widget respectively
// design choice: 
// 1) having a single vector of widgets that will encapsulate all the stateless and statefull
// widgets
// pros: 
//  - having all widgets encapsulated in a one variable
//  - a single variable is reposable for deaing with all widgets
// cons:
//  - having to check each time we deal with a widget for it's type
// 2) having a separate vecctor for stateless and statefull widgets respetively
// Answer:
// the screen will be only concerned about the statefull widgets, since we can
// have a statefull widget that behave like stateless one, and not the other
// way around
#[derive(Debug)]
pub struct WidgetWrapper<'a, T: StatefulWidget>{
    wrapped: T,
    up: Option<&'a mut T>,
    down: Option<&'a mut T>,
    right: Option<&'a mut T>,
    left: Option<&'a mut T>,
}

#[derive(Debug, Default)]
pub struct StatetWrapper<'a>{
    pub wrapped: State,
    pub up: Option<&'a mut StatetWrapper<'a>>,
    pub down: Option<&'a mut StatetWrapper<'a>>,
    pub right: Option<&'a mut StatetWrapper<'a>>,
    pub left: Option<&'a mut StatetWrapper<'a>>,
}

/// compounds the state with the equivelent rectangle
#[derive(Debug)]
pub struct Compounder<'a>{
    pub states: Vec<(StatetWrapper<'a>, Rect)>
}

impl<'a> Compounder<'a> {

    pub fn new(states: Vec<(StatetWrapper<'a>, Rect)>) -> Compounder{
        Compounder { states }
    }

    pub fn encapsulate(inner: Vec<(State, Rect)>)->Compounder<'a>{

        let mut toreturn = vec![];

        for (state, rect) in inner.iter()  {
            
            let temp_state = state.clone();

            toreturn.push((
                StatetWrapper{
                    wrapped: temp_state,
                    up: None,
                    down: None,
                    left: None,
                    right: None,
                },
                rect.clone()
            ))
            
        }

        Compounder::new(toreturn)

    }

    /// extract the corresponding rectangle of a widget from the config file
    /// under development
    pub fn get_rect(conf: &Config, widget: String, term: &mut Terminal<CrosstermBackend<Stdout>>) -> Rect{

        let mut toreturn = Rect { x: 0, y: 0, width: 0, height: 0 };

        let binding = term.get_frame();
        let mut fixer = Fixer::new(&binding);

        let mut h_flag = false;
        let mut w_flag = false;
        let mut x_flag = false;
        let mut y_flag = false;

        match conf.conf.to_owned() {

            toml::Value::Table(values)=>{

                for (key, value) in values {

                    if key.to_string() == widget {

                        match value{

                            toml::Value::Table(v)=>{

                                for (key, value) in v {

                                    match key.as_str() {
                                        "x" =>{
                                            match  value.as_float() {
                                                Some(X) => {
                                                    let conv = X * 100 as f64;
                                                    toreturn.x = fixer.xratio(conv as u16);
                                                },
                                                None => {x_flag = true;}
                                            }

                                            if x_flag {

                                                match value.as_integer() {
                                                    Some(X) => {
                                                        toreturn.x = X as u16;
                                                    },
                                                    None => {panic!("error parsing the 'x' property of [Button]")}
                                                }

                                            }
                                        }
                                        "y"=>{
                                            match value.as_float() {
                                                Some(Y) => {
                                                    let conv = Y * 100 as f64;
                                                    toreturn.y  = fixer.yratio(conv as u16);
                                                },
                                                None => {y_flag = true;}
                                            }

                                            if y_flag {

                                                match value.as_integer() {
                                                    Some(Y) => {
                                                        toreturn.y = Y as u16;
                                                    },
                                                    None => {panic!("error parsing the 'y' property of [Button]")}
                                                }

                                            }
                                        }

                                        "width"=>{
                                            match  value.as_float() {

                                                Some(W) => {
                                                    let conv = W * 100 as f64;
                                                    toreturn.width = fixer.wratio(conv as u16);
                                                },
                                                None => {

                                                    w_flag = true;
                                                }
                                            }

                                            if w_flag {

                                                match value.as_integer() {
                                                    Some(W) => {
                                                        toreturn.width = W as u16;
                                                    },
                                                    None => {panic!("error parsing the 'width' property of [Button]")}
                                                }

                                            }
                                        }
                                        "height"=>{
                                            match  value.as_float() {
                                                Some(H) => {
                                                    let conv = H * 100 as f64;
                                                    toreturn.height = fixer.hratio(conv as u16);
                                                },
                                                None => {
                                                    h_flag = true;
                                                }
                                            }

                                            if h_flag {

                                                match value.as_integer() {
                                                    Some(H) => {
                                                        toreturn.height = H as u16;
                                                    },
                                                    None => {panic!("error parsing the 'height' property of [Button]")}
                                                }

                                            }
                                        }
                                        _=>{}
                                    }
                                    
                            _=>{}

                        }
                        
                    }
                    
                }

            },
            _=>{}
            
        }

        toreturn

    }

    /// Sorts the compounder respect to the Rect field
    /// so that each wrapper will link to another wrapper of the same vector
    /// TODO
    pub fn sort(&mut self){}
    
}

mod Test{
    #[test]
    fn should_sort() {}

    #[test]
    fn rect_construction() {

        let mut terminal = Terminal::new(CrosstermBackend::new(stdout())).unwrap();
        let conf = Config::read();
        let rect = Compounder::get_rect(&conf, String::from("Timer"), &mut terminal);

    }
}
