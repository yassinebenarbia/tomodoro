/**
 *
* TODO: make the button respond to the click events
* TODO: make the button display a custom banner for each state
* TODO: make an option to custimize each banner for each state
* TODO: add an option to toggle the click behavior
* TODO: add an option to toggle the state display behavior
* TODO: clean up the code
* TODO: upload the thing to a public repo
* TODO: add the a sound when the focus duration ends
* TODO: add the a sound when the rest duration ends
* TODO: add option to toggle the focus duration sound
* TODO: add option to toggle the rest duration sound
* TODO: add a cycles counter that displayes the number of cycles
* TODO: add an option to toggle the visibility of each object
**/
use std::io;
use once_cell::sync::Lazy;
use std::{
    time::{
        Duration, Instant, SystemTime, UNIX_EPOCH
    },
    error::Error, io::Stdout, thread
};
use crossterm::{
    terminal::{
        enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode
    },
    execute, event::{
        EnableMouseCapture, DisableMouseCapture, KeyCode, Event, self
    }, 
};
use tui::{
    backend::{Backend, CrosstermBackend},
    Frame, Terminal,
};
use crate::{
    commands::{Commands, CommandSetter},
    screen::Compounder,
    config::Config, constructor::{
        self, construct_timer_state, construct_button_state,
        construct_timer_rect, construct_button_rect
    },
};

pub static mut COMMAND: Commands = Commands::Start;
pub static mut PAUSED_DURATION:Duration = Duration::ZERO ;
pub static mut SMALL_PAUSED_DURATION:Duration = Duration::ZERO ;
pub static mut PAUSED_START_TIME: Lazy<Duration> = Lazy::new(||{SystemTime::now().duration_since(UNIX_EPOCH).unwrap()});
pub static mut CYCLES: u64 = 0;
pub static mut QUIT: bool = false;
pub static mut PHASE: &str = "focus";

pub struct App {}

impl App {
    pub fn renderui<'a, B>(
        f: & mut Frame<'a ,B>,
        conf: &Config,
        comp : &mut Compounder
    ) where
        B: Backend,
    {
        let stdout = io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend).unwrap();

        let timer = constructor::construct_timer(
            &mut terminal,
            conf
        );

        let button = constructor::construct_button(
            &mut terminal,
            conf
        );


        let button_layout = button.layout.clone();
        let timer_layout = timer.layout.clone();

        if let Some(value) = comp.states.get_mut(0){
            let state = &mut value.0;
            unsafe{
                match COMMAND {
                    Commands::Stop=>{
                        state.states.insert("working".to_string(), "false".to_string());
                    },
                    Commands::Start=>{
                        state.states.insert("working".to_string(), "true".to_string());
                    },
                }
            }

            f.render_stateful_widget(
                timer,
                timer_layout,
                state,
            );
        }

        if let Some(value) = comp.states.get_mut(1){
            let state = &mut value.0;
            unsafe{
                match COMMAND {
                    Commands::Stop=>{
                        state.states.insert("working".to_string(), "false".to_string());
                    },
                    Commands::Start=>{
                        state.states.insert("working".to_string(), "true".to_string());
                    },
                }
            }

            f.render_stateful_widget(
                button,
                button_layout,
                state
                );
        }
    }

    pub fn run<'a>(self) -> Result<(), Box<dyn Error>>{
        // setup terminal
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        
        // run the app
        let last_tick = Instant::now();

        let conf = Config::read();
        let r = terminal.size().unwrap();

        let timer_rect = construct_timer_rect(&conf, &r);
        let button_rect = construct_button_rect(&conf, &r);

        let timer_state = construct_timer_state(&conf, &mut terminal);
        let button_state = construct_button_state(&conf, &mut terminal);
        
        let load = vec![
            (timer_state, timer_rect),
            (button_state, button_rect)
        ];

        let mut comp = Compounder::encapsulate(load);

        loop {
            unsafe{
                if QUIT {
                    return App::quit(terminal);
                }
            }

            // avoid overheading the system 
            let timeout = Duration::from_millis(250)
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if crossterm::event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char('q') =>{
                            return App::quit(terminal);
                        },
                        KeyCode::Char(' ') =>{
                            App::command(CommandSetter::Revert);
                        },
                        _ => {}
                    }
                }
            }

            terminal.draw(|f| {
                App::renderui::<CrosstermBackend<Stdout>>(f,
                    &conf,
                    &mut comp
                );
            })?;

            // Note: "my cpu is overheating, will remove that after performance
            // optimization : ("
            thread::sleep(Duration::from_millis(150));
        }
    }

    pub fn command(action: CommandSetter){
        unsafe{
            match action {
                CommandSetter::Start => {
                    COMMAND = Commands::Start;
                    PAUSED_DURATION += SMALL_PAUSED_DURATION;
                }
                CommandSetter::Stop=>{
                    COMMAND = Commands::Stop;
                    PAUSED_START_TIME = Lazy::new(||{SystemTime::now().duration_since(UNIX_EPOCH).unwrap()});
                }
                CommandSetter::Revert=>{
                    match COMMAND{
                        // meaning that the timer will STOP
                        Commands::Start => {
                            COMMAND = Commands::Stop;
                            PAUSED_START_TIME = Lazy::new(||{SystemTime::now().duration_since(UNIX_EPOCH).unwrap()});
                        }
                        // meaning that the timer will START
                        Commands::Stop=>{
                            COMMAND = Commands::Start;
                            PAUSED_DURATION += SMALL_PAUSED_DURATION;
                        }
                    }
                }
            }
        }
    }

    pub fn quit(mut terminal:Terminal<CrosstermBackend<Stdout>>) -> Result<(), Box<dyn Error>>{
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture,
        )?;

        disable_raw_mode().unwrap();

        terminal.show_cursor()?;

        Ok(())
    }
}

impl Default for App{
    fn default() -> Self {
        App {}
    }
}
