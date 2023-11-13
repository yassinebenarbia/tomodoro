use std::{fs::OpenOptions, io::Stdout};
use std::io::Write;

use toml::{Value, value::Time};
use tui::widgets::{Borders, BorderType};
use tui::{widgets::StatefulWidget, style::Style, Terminal, backend::CrosstermBackend};

use crate::{displayable::Displayable, config::Config, State::State, statefull_timer::Timer, stateful_button::StatefullButton, timer_widget::TimerWidget, capabilities::{hex_to_rgb, is_float, is_number}, widget_fixer::Fixer};

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
    fn render(self, area: tui::layout::Rect, buf: &mut tui::buffer::Buffer, state: &mut Self::State) {
        
    }
    
}

impl StatefulWidget for Thingy {

    type State = State;
    fn render(self, area: tui::layout::Rect, buf: &mut tui::buffer::Buffer, state: &mut Self::State) {
        
    }
    
}

impl Displayable for Thingy{

    fn x(&self) -> u16 {self.x}

    fn y(&self) -> u16 {self.y}

    fn width(&self) -> u16 {0}

    fn height(&self) -> u16 {0}

    fn highlight(&self) {}

    fn manage_state(&self, state: &mut crate::State::State) {
        
    }

}

impl Displayable for Dumy {

    fn x(&self) -> u16 {
        self.x
    }

    fn y(&self) -> u16 {
        self.y
    }

    fn width(&self) -> u16 {
        0
    }

    fn height(&self) -> u16 {
        0
    }

    fn highlight(&self) {}

    fn manage_state(&self, state: &mut crate::State::State) {}

}


/// construct a vector of widgets based on the `widgets parameter`, where
/// each widget implements the `Displayable trait
/// TODO this should also provide the states
pub fn constructor<'a>(config: &Config, term: &mut Terminal<CrosstermBackend<Stdout>>) -> Vec<Box<dyn Displayable>>{

    let prim_widgets = config.filter(&vec!["Timer", "Button"]);

    construct_Timer(config);

    truck(config, term);

    return vec![Box::new(Timer::default()), Box::new(StatefullButton::default())];

}

pub fn truck(conf: &Config, term: &mut Terminal<CrosstermBackend<Stdout>>)-> Vec<(Box<State>, Box<dyn Displayable>)>{

    let timer_string = String::from("Timer");
    let button_string = String::from("Button");
    let toreturn: Vec<(Box<State>, Box<dyn Displayable>)>;

    match conf.conf.clone() {

        toml::Value::Table(table) => {

            for (key, val) in table.iter() {

                match key.as_str() {

                    timer_string => {
                        timer_construct(val, term);
                        // toreturn.push(timer_state_consturct(val), timer_construct(val))
                    },
                    button_string => {
                        button_construct(val);
                        // toreturn.push(button_state_consturct(val), button_construct(val))
                    }
                    _ => {}

                }

            }

        }

        _ => {}

    }

    return vec![
        (Box::new(State::default()), Box::new(Timer::default())),
        (Box::new(State::default()), Box::new(StatefullButton::default())),
    ];

}

//TODO construct the function timer_state_constructor
//TODO construct the function button_constructor
//TODO construct the function button_state_constructor

/// Constructs the `Timer` widget based on the `Values` provided on the\
/// `values` parameter.
/// returns a `Timer`
fn timer_construct(values:& Value, term: &mut Terminal<CrosstermBackend<Stdout>>) -> Timer{

    // in case one of the fields of the config isn't of type
    // float, we flag it as true to check for integer or else
    // it panic
    let mut h_flag = false;
    let mut w_flag = false;
    let mut x_flag = false;
    let mut y_flag = false;

    // desired timer and it's internals
    let toreturn = Timer::default();
    let timer_widget = TimerWidget::default();
    let style = Style::default();


    let binding = term.get_frame();
    let mut fixer = Fixer::new(&binding);

    let mut width: u16 = fixer.wratio(30);
    let mut height: u16 = fixer.hratio(20);
    let mut x: u16 = fixer.xratio(30);
    let mut y: u16 = fixer.xratio(20);

    match values {

        toml::Value::Table(table) => {

            for (key, value) in table {

                match key.as_str() {

                    "color" => {
                        // we want the application to panic if the provided color is not correct
                        // and to give the default rgb colros (100, 100, 100)
                        let (r,g,b) = hex_to_rgb(value.as_str().expect(
                            "The 'color' value is not valid, please consider checking the config file \
                              under [Timer]"
                        )).unwrap_or_else(|| (100,100,100));

                        style.fg(tui::style::Color::Rgb(r, g, b));
                    },

                    "background_color" => {
                        // we want the application to panic if the provided color is not correct
                        let (r,g,b) = hex_to_rgb(value.as_str().expect(
                            "The 'background color' value is not valid, please consider checking the config file \
                              under [Timer]"
                        )).unwrap_or_else(|| (0,0,0));

                        style.fg(tui::style::Color::Rgb(r, g, b));
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
                            println!("{}", value.as_integer().unwrap());

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
                                y = fixer.xratio(conv as u16);
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

            toreturn.clone()
                .layout(x, y, width, height)
                .widget(
                    timer_widget.style(style)
                        .borders(Borders::ALL)
                        .border_type(BorderType::Double)
                );

        }

        _ => {}

    }

    return toreturn;

}

fn button_construct(values:& Value){

}

/// cosntructs a `Timer` instance from the provided config
fn construct_Timer(config: &Config) -> Timer{

    let mut file = OpenOptions::new().append(true).open("log").unwrap();

    // check the `[Timer]` field in the config file
    match config.conf.get("Timer").expect("Timer is not provided in the config file") {

        // check the vlaues of the `[Timer]` table
        toml::Value::Table(table) => {

            // for each key value paires under the `[Timer]` field
            for (key, val) in table.iter() {

                file.write(format!("Key: {}, Value: {}\n", key, val).as_bytes());

                // match key {
                //     // std::string::String::from("hello") => {}
                // };

            }

        },

        _ => {}
        
    };

    Timer::default()
}


mod Test{
    use std::io::stdout;

    use crossterm::terminal;
    use tui::{Terminal, backend::CrosstermBackend};

    use crate::config::Config;

    use super::constructor;

    #[test]
    fn construction() {

        let tconfig = toml::de::from_str(r#"
            [Timer]
              color = '#000000'
              width = 0.5
              height = 0.4
              x = 10
              y = 40
            [Button]
              color = '#000000'
              width = 0.5
              height = 0.4
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
        println!("from constructor:\n{:?}", constructor(&conf, &mut terminal));

    }

}
