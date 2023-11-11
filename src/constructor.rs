use tui::widgets::StatefulWidget;

use crate::{displayable::Displayable, config::Config, State::State, statefull_timer::Timer, stateful_button::StatefullButton};

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
pub fn constructor<'a>(widgets: &Config) -> 
Vec<Box<dyn Displayable>>{

    let prim_widgets = widgets.filter(&vec!["Timer", "Button"]);

    return vec![Box::new(Timer::default()), Box::new(StatefullButton::default())];

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
