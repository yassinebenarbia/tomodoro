use std::fs::OpenOptions;
use std::io::Write;

use tui::widgets::StatefulWidget;

use crate::{displayable::Displayable, config::Config, State::State, statefull_timer::Timer, stateful_button::StatefullButton};

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
pub fn constructor<'a>(config: &Config) -> 
Vec<Box<dyn Displayable>>{

    let prim_widgets = config.filter(&vec!["Timer", "Button"]);
    construct_Timer(config);

    return vec![Box::new(Timer::default()), Box::new(StatefullButton::default())];

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

pub fn truck(conf: &Config)-> Vec<(Box<State>, Box<dyn Displayable>)>{

    match conf.conf.clone() {

        toml::Value::Table(table) => {
            for (key, val) in table.iter() {
                println!("Key: {}", key);
            }
        }

        toml::Value::Array(array) => {
            for val in array.iter() {
            }
        }

        toml::Value::Integer(int_val) => {
            println!("Integer Value: {}", int_val);
        }

        toml::Value::Float(float_val) => {
            println!("Float Value: {}", float_val);
        }

        toml::Value::String(str_val) => {
            println!("String Value: {}", str_val);
        }

        toml::Value::Boolean(bool_val) => {
            println!("Boolean Value: {}", bool_val);
        }

        _ => {
            // Handle other types if necessary
        }

    }

    return vec![
        (Box::new(State::default()), Box::new(Timer::default())),
        (Box::new(State::default()), Box::new(StatefullButton::default())),
    ];
}

mod Test{
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

        println!("from constructor:\n{:?}", constructor(&conf));

    }

}
