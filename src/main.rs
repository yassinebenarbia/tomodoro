use std::io;

use app::App;

mod state;
mod config;
mod constructor;
mod commands;
mod displayable;
mod screen;
mod state_displayer;
mod frame_util;
mod stateful_timer;
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
        Err(e)=>println!("Error occured: {}", e)
    }

    Ok(())
}
