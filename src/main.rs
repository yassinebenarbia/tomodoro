use std::io;

use app::App;

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
mod state;

fn main() -> Result<(), io::Error> {

    let app = App::default();
    let state = app.run();

    match state {
        Ok(_)=>println!("terminated successfully"),
        Err(_)=>println!("Oh no! an Error occured")
    }

    Ok(())
}
