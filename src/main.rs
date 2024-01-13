use std::io;

use app::App;

mod trait_holder;
mod State;
mod config;
mod constructor;
mod directions;
mod displayable;
mod screen;
mod widget_fixer;
mod state_displayer;
mod frame_util;
mod statefull_timer;
mod timer_state; 
mod timer_widget;
mod button_widget;
mod stateful_button;
mod button;
mod app;
mod capabilities;
mod player;

fn main() -> Result<(), io::Error> {

    let app = App::default();
    let state = app.run();

    match state {
        Ok(_)=>println!("terminated successfully"),
        Err(_)=>println!("Oh no! an Error occured")
    }

    Ok(())
}
