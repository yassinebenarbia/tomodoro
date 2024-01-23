use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use std::{io::Stdout};

use toml::{Value};
use tui::layout::Rect;
use tui::widgets::{Borders, BorderType};
use tui::{widgets::StatefulWidget, style::Style, Terminal, backend::CrosstermBackend};

use crate::button_widget::ButtonWidget;
use crate::stateful_button::Button;
use crate::{ config::Config, state::State, statefull_timer::Timer, timer_widget::TimerWidget, capabilities::{hex_to_rgb}, widget_fixer::Fixer};

pub struct Constructor;

#[derive(Debug, Clone)]
pub struct Dumy{
    x: u16,
    y: u16,
}


trait States {}

pub struct State1{}

impl<T: States> States for T{}


#[derive(Debug, Clone)]
pub struct Thingy{
    x: u16,
    y: u16,
}

impl Thingy {

    pub fn new(x: u16, y: u16) -> Thingy{
        Thingy { x, y }
    }

}

impl Dumy {

    pub fn new(x: u16, y: u16) -> Dumy {
        Dumy { x, y }
    }

}


impl StatefulWidget for Dumy {

    type State = State;
    fn render(self, _area: tui::layout::Rect, _buf: &mut tui::buffer::Buffer, _state: &mut Self::State) {
        
    }
    
}

impl StatefulWidget for Thingy {

    type State = State;
    fn render(self, _area: tui::layout::Rect, _buf: &mut tui::buffer::Buffer, _state: &mut Self::State) {
        
    }
    
}


/// construct a vector of widgets based on the `widgets parameter`, where
/// each widget implements the `Displayable trait
/// TODO this should also provide the states

pub fn truck(conf: &Config, term: &mut Terminal<CrosstermBackend<Stdout>>)->
Vec<(Box<dyn StatefulWidget<State = State>>, Box<Rect>, Box<State>)>{

    let _timer_string = String::from("Timer");
    let _button_string = String::from("Button");

    let mut timer = Timer::default();
    let mut timer_state = State::default();
    let mut button = Button::default();
    let mut button_state = State::default();

    match conf.conf.clone() {

        toml::Value::Table(table) => {

            for (key, val) in table.iter() {

                match key.as_str() {

                    _timer_string => {
                        timer = construct_timer(val, term);
                        timer_state = construct_timer_state(val, term);
                        // toreturn.push(timer_state_consturct(val), timer_construct(val))
                    },
                    _button_string => {
                        button = construct_button(val, term);
                        button_state = construct_button_state(val, term);
                        // toreturn.push(button_state_consturct(val), button_construct(val))
                    }
                    _ => {}

                }

            }

            // return vec![
            //     (Box::new(timer), Box::new(timer_state)), 
            //     (Box::new(button), Box::new(button_state))
            // ];

            let timer_layout = timer.layout.clone();
            let button_layout = button.layout.clone();

            return vec![
                (Box::new(timer),Box::new(timer_layout), Box::new(timer_state)), 
                (Box::new(button), Box::new(button_layout) ,Box::new(button_state))
            ];

            // return vec![
            //     (Box::new(timer), Box::new(timer_layout), Box::new(timer_state)), 
            //     (Box::new(button), Box::new(timer_layou), Box::new(button_state))
            // ];


        }

        _ => {}

    }

    return vec![
        (Box::new(Timer::default()), Box::new(Timer::default().layout), Box::new(State::default())),
        (Box::new(Button::default()), Box::new(Timer::default().layout), Box::new(State::default())),
    ];

}

/// Constructs the `Timer` widget based on the `Values` provided on the `values` parameter.\
/// returns a `Timer`
pub fn construct_timer(values:& Value, term: &mut Terminal<CrosstermBackend<Stdout>>) -> Timer{

    // in case one of the fields of the config isn't of type
    // float, we flag it as true to check for integer or else
    // it panic
    let mut h_flag = false;
    let mut w_flag = false;
    let mut x_flag = false;
    let mut y_flag = false;

    // desired timer and it's internals
    let mut toreturn = Timer::default();

    let mut timer_widget = TimerWidget::default();
    let mut style = Style::default();


    let binding = term.get_frame();
    let mut fixer = Fixer::new(&binding);

    let mut width: u16 = fixer.wratio(30);
    let mut height: u16 = fixer.hratio(20);
    let mut x: u16 = fixer.xratio(30);
    let mut y: u16 = fixer.xratio(20);

    match values {

        // check if it's a table
        toml::Value::Table(table) => {

            for (k, v) in table {

                match k.as_str() {

                    // check for the Timer key
                    "Timer"=>{

                        match v {

                            // the table of key values under the timer key
                            toml::Value::Table(v) =>{

                                for (key, value) in v{

                                    match key.as_str() {

                                        "color" => {
                                            // we want the application to panic if the provided color is not correct
                                            // and to give the default rgb colros (100, 100, 100)
                                            let (r,g,b) = hex_to_rgb(value.as_str().expect(
                                                "The 'color' value is not valid, please consider checking the config file \
                                                    under [Timer]"
                                            )).unwrap_or_else(|| (100,100,100));

                                            style = style.fg(tui::style::Color::Rgb(r, g, b));
                                        },

                                        "background_color" => {
                                            // we want the application to panic if the provided color is not correct
                                            let (r,g,b) = hex_to_rgb(value.as_str().expect(
                                                "The 'background color' value is not valid, please consider checking the config file \
                                                    under [Timer]"
                                            )).unwrap_or_else(|| (0,0,0));

                                            style = style.bg(tui::style::Color::Rgb(r, g, b));
                                        },

                                        "width" => {
                                            match  value.as_float() {

                                                Some(W) => {
                                                    let conv = W * 100 as f64;
                                                    width = fixer.wratio(conv as u16);
                                                },
                                                None => {

                                                    w_flag = true;
                                                }
                                            }

                                            if w_flag {

                                                match value.as_integer() {
                                                    Some(W) => {
                                                        width = W as u16;
                                                    },
                                                    None => {panic!("error parsing the 'width' property of [Timer]")}
                                                }

                                            }

                                        },
                                        "height" => {

                                            // if the provided height is a percentage
                                            match value.as_float() {
                                                Some(H) => {
                                                    let conv = H * 100 as f64;
                                                    height = fixer.hratio(conv as u16);
                                                },
                                                None => {
                                                    h_flag = true;
                                                }
                                            }

                                            if h_flag {

                                                match value.as_integer() {
                                                    Some(H) => {
                                                        height = H as u16;
                                                    },
                                                    None => {panic!("error parsing the 'height' property of [Timer]")}
                                                }

                                            }

                                        },
                                        "x" => {

                                            match  value.as_float() {
                                                Some(X) => {
                                                    let conv = X * 100 as f64;
                                                    x = fixer.xratio(conv as u16);
                                                },
                                                None => {x_flag = true;}
                                            }

                                            if x_flag {

                                                match value.as_integer() {
                                                    Some(X) => {
                                                        x = X as u16;
                                                    },
                                                    None => {panic!("error parsing the 'x' property of [Timer]")}
                                                }

                                            }

                                        },
                                        "y" => {

                                            match value.as_float() {
                                                Some(Y) => {
                                                    let conv = Y * 100 as f64;
                                                    y = fixer.yratio(conv as u16);
                                                },
                                                None => {y_flag = true;}
                                            }

                                            if y_flag {

                                                match value.as_integer() {
                                                    Some(Y) => {
                                                        y = Y as u16;
                                                    },
                                                    None => {panic!("error parsing the 'y' property of [Timer]")}
                                                }

                                            }
                                        }
                                        _ => {}
                                    }

                                }

                            }

                            _=>{}
                            
                        }

                    }
                    _=>{}
                }
                
            }

            timer_widget.style = style;
            timer_widget.borders = Borders::ALL;
            timer_widget.border_type = BorderType::Rounded;

            return toreturn
                .layout(x, y, width, height)
                .widget(timer_widget).to_owned();

        }

        _ => {}

    }

    return toreturn;

}

/// Constructs the `StatefullButton` widget based on the `Values` provided on the `values` parameter.
/// returns a `StatefullButton`
pub fn construct_button<'b>(values:& Value, term: &mut Terminal<CrosstermBackend<Stdout>>)
    -> Button<'b>
{

    let mut toreturn = Button::default();
    let mut style = Style::default();
    let mut button_widget = ButtonWidget::default();

    let mut h_flag = false;
    let mut w_flag = false;
    let mut x_flag = false;
    let mut y_flag = false;

    let binding = term.get_frame();
    let mut fixer = Fixer::new(&binding);

    let mut width: u16 = fixer.wratio(30);
    let mut height: u16 = fixer.hratio(20);
    let mut x: u16 = fixer.xratio(40);
    let mut y: u16 = fixer.xratio(20);

    match values {

        // if the there is a table 
        toml::Value::Table(table) => {

            for (key, value) in table {

                match key.as_str() {

                    // the table head is a "Button"
                    "Button"=>{

                        match value {

                            toml::Value::Table(v)=>{

                                for (key, value) in v {

                                    match key.as_str() {

                                        "color" => {
                                            // we want the application to panic if the provided color is not correct
                                            // and to give the default rgb colros (100, 100, 100)
                                            let (r,g,b) = hex_to_rgb(value.as_str().expect(
                                                "The 'color' value is not valid, please consider checking the config file \
                                                    under [Button]"
                                            )).unwrap_or_else(|| (100,100,100));

                                            style = style.fg(tui::style::Color::Rgb(r, g, b));
                                        },

                                        "background_color" => {
                                            // we want the application to panic if the provided color is not correct
                                            let (r,g,b) = hex_to_rgb(value.as_str().expect(
                                                "The 'background color' value is not valid, please consider checking the config file \
                                                    under [Button]"
                                            )).unwrap_or_else(|| (50,50,50));

                                            style = style.bg(tui::style::Color::Rgb(r, g, b));
                                        },

                                        "width" => {
                                            match  value.as_float() {

                                                Some(W) => {
                                                    let conv = W * 100 as f64;
                                                    width = fixer.wratio(conv as u16);
                                                },
                                                None => {

                                                    w_flag = true;
                                                }
                                            }

                                            if w_flag {

                                                match value.as_integer() {
                                                    Some(W) => {
                                                        width = W as u16;
                                                    },
                                                    None => {panic!("error parsing the 'width' property of [Button]")}
                                                }

                                            }

                                        },
                                        "height" => {

                                            match  value.as_float() {
                                                Some(H) => {
                                                    let conv = H * 100 as f64;
                                                    height = fixer.hratio(conv as u16);
                                                },
                                                None => {
                                                    h_flag = true;
                                                }
                                            }

                                            if h_flag {

                                                match value.as_integer() {
                                                    Some(H) => {
                                                        height = H as u16;
                                                    },
                                                    None => {panic!("error parsing the 'height' property of [Button]")}
                                                }

                                            }

                                        },
                                        "x" => {

                                            match  value.as_float() {
                                                Some(X) => {
                                                    let conv = X * 100 as f64;
                                                    x = fixer.xratio(conv as u16);
                                                },
                                                None => {x_flag = true;}
                                            }

                                            if x_flag {

                                                match value.as_integer() {
                                                    Some(X) => {
                                                        x = X as u16;
                                                    },
                                                    None => {panic!("error parsing the 'x' property of [Button]")}
                                                }

                                            }

                                        },
                                        "y" => {

                                            match value.as_float() {
                                                Some(Y) => {
                                                    let conv = Y * 100 as f64;
                                                    y = fixer.yratio(conv as u16);
                                                },
                                                None => {y_flag = true;}
                                            }

                                            if y_flag {

                                                match value.as_integer() {
                                                    Some(Y) => {
                                                        y = Y as u16;
                                                    },
                                                    None => {panic!("error parsing the 'y' property of [Button]")}
                                                }

                                            }
                                        }
                                        _ => {}

                                    }


                                }

                            }

                            _=>{}

                        }

                    }
                    _=>{}
                    
                }


                
            }

            // TODO need to implement the onclick behavior
            // toreturn.widget.style = style;
            // toreturn.widget.title = Some(title.into());

            button_widget.style = style;

            toreturn
                .layout(x, y, width, height)
                .widget(button_widget);

            return toreturn;


        },

        _ => {}

    }

    return toreturn;

}

/// construcuts `State` from the `values` paramater, or in another word
/// from the config file
/// This state is specifically desined for the `Timer` widget
pub fn construct_timer_state(values:& Value, _term: &mut Terminal<CrosstermBackend<Stdout>>) -> State{

    let mut state = State::default();
    let mut timer_hashmap = HashMap::new();

    timer_hashmap.insert("displayed".to_string(), 20.to_string());
    timer_hashmap.insert("focus_duration".to_string(), 1500.to_string());
    timer_hashmap.insert("rest_duration".to_string(), 300.to_string());
    timer_hashmap.insert("cycles".to_string(), 1.to_string());
    timer_hashmap.insert("max_cycles".to_string(), "inf".to_string());
    timer_hashmap.insert("prev_diff".to_string(), 0.to_string());
    let current = SystemTime::now().duration_since(UNIX_EPOCH).expect("Could not get the current time").as_secs().to_string();
    timer_hashmap.insert("start".to_string(), current);
    timer_hashmap.insert("working".to_string(), "true".to_string());
    timer_hashmap.insert("phase".to_string(), "focus".to_string());
    timer_hashmap.insert("focus_alarm".to_string(), "".to_string());
    timer_hashmap.insert("rest_alarm".to_string(), "".to_string());
    

    // TODO add support of the working state for the config file
    match values{

        toml::Value::Table(table) => {

            for (key, value) in table {

                match key.as_str() {

                    "Timer" => {

                        match value {

                            toml::Value::Table(v)=>{

                                for (key, value ) in v {

                                    match key.as_str() {
                                        "rest_duration" => {
                                            timer_hashmap.insert(key.to_string(), value.to_string());
                                        },
                                        "focus_duration" => {
                                            timer_hashmap.insert(key.to_string(), value.to_string());
                                            timer_hashmap.insert("displayed".to_string(), value.to_string());
                                        },
                                        "cycles" => {
                                            timer_hashmap.insert(key.to_string(), value.to_string());
                                        },
                                        "max_cycles" => {
                                            timer_hashmap.insert(key.to_string(), value.to_string());
                                        },
                                        "focus_alarm" => {
                                            timer_hashmap.insert(key.to_string(), value.as_str().to_owned().unwrap().to_string());
                                        },
                                        "rest_alarm" => {
                                            timer_hashmap.insert(key.to_string(), value.as_str().to_owned().unwrap().to_string());
                                        },
                                        _ => {}

                                    }

                                }

                            }
                            _ =>{}
                            
                        }

                    },

                    _ => {}


                }

            }

        },

        _ => {}

    };

    state.states = timer_hashmap;
    state

}

pub fn construct_button_state(values:& Value, _term: &mut Terminal<CrosstermBackend<Stdout>>) -> State{

    let mut state = State::default();
    let mut button_hasmap = HashMap::new();

    button_hasmap.insert("clicked".to_string(), "false".to_string());
    button_hasmap.insert("hovered".to_string(), "false".to_string());

    button_hasmap.insert("focus_banner".to_string(), "focus".to_string());
    button_hasmap.insert("rest_banner".to_string(), "rest".to_string());
    button_hasmap.insert("pause_banner".to_string(), "pause".to_string());
    button_hasmap.insert("clickable".to_string(), "false".to_string());

    match values{

        toml::Value::Table(table) => {

            for (key, value) in table {

                match key.as_str() {

                    "Button" => {

                        match value {

                            toml::Value::Table(v)=>{

                                for (key, value ) in v {

                                    match key.as_str() {
                                        "rest_banner" => {
                                            button_hasmap.insert(key.to_string(), value.to_string());
                                        },
                                        "focus_banner" => {
                                            button_hasmap.insert(key.to_string(), value.to_string());
                                        },
                                        "pause_banner" => {
                                            button_hasmap.insert(key.to_string(), value.to_string());
                                        },
                                        "clickable" => {
                                            button_hasmap.insert(key.to_string(), value.to_string());
                                        }
                                        _ => {}

                                    }

                                }

                            }
                            _ =>{}

                        }

                    },

                    _ => {}


                }

            }

        },

        _ => {}

    };


    state.states = button_hasmap;

    return state;
}

mod Test{
    
    #![allow(unused_imports)]
    use std::io::stdout;

    use crossterm::terminal;
    use tui::{Terminal, backend::CrosstermBackend};

    use crate::config::Config;

    #[test]
    fn construction() {

        let tconfig = toml::de::from_str(r#"
            [Timer]
              color = '#000000'
              width = 0.5
              height = 0.2
              x = 10
              y = 10
            [Button]
              color = '#000000'
              width = 0.5
              height = 0.3
              x = 20
              y = 10
            [Widget]
              color = '#000000'
              width = 0.5
              height = 0.4
              x = 0
              y = 10
        "#).unwrap();

        let conf = Config {
            conf: tconfig,
        };

        let mut terminal = Terminal::new(CrosstermBackend::new(stdout())).unwrap();

    }

}
