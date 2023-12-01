use std::{time::{Duration, Instant, SystemTime, UNIX_EPOCH}, error::Error, fmt::{Alignment, Debug}, io::{Stdout, Write}, ops::{Deref, Add}, fs::OpenOptions};
use std::io;
use crossterm::{terminal::{enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}, execute, event::{EnableMouseCapture, DisableMouseCapture, KeyCode, Event, self}, Command };
use once_cell::sync::Lazy;
use tui::{
    backend::{Backend, CrosstermBackend},
    widgets::{Borders, StatefulWidget},
    layout::{Rect, Direction},
    Frame, style::{Style, Color, Modifier}, buffer::Buffer, Terminal,
};

use crate::{
    stateful_button::{Button, ButtonState}
    ,button::Cadre, button_widget::ButtonWidget, statefull_timer::Timer,
    timer_widget::TimerWidget, timer_state::TimerState, widget_fixer::Fixer,
    displayable::Displayable, screen::{ self, Compounder, StatetWrapper, WidgetWrapper}, config::Config, directions::{Commands, self, command_setter}, constructor::{ truck, self, construct_timer_state, construct_button_state}, State, state, trait_holder::TraitHolder
};
use std::sync::Once;

static INIT: Once = Once::new();

pub static mut command: Commands = Commands::Start;
pub static mut paused_duration: Duration = Duration::ZERO;
pub static mut paused_start: Lazy<Duration> = Lazy::new(||{SystemTime::now().duration_since(UNIX_EPOCH).unwrap()});

/// widget
fn get_block<'a>(title: String) -> ButtonWidget<'a>{
    return ButtonWidget::default()
        .style(
            Style::default()
            .fg(Color::Red).bg(Color::Cyan)
            .add_modifier(Modifier::BOLD | Modifier::ITALIC)
        )
        .title(title.clone()).title_alignment(Alignment::Center)
        .borders(Borders::ALL);
}

// #[derive(Debug, Clone)]
pub struct Dumy<'a>{
    field: Vec<(StatetWrapper<'a>, Rect)>
}

impl<'a> Dumy<'a> {
    pub fn new(field: Vec<(StatetWrapper<'a>, Rect)>) -> Dumy<'a> {
        Dumy {field}
    }

    pub fn construct(material: Vec<(State::State, Rect)>) -> Compounder<'a>{

        let mut toreturn = vec![];

        for (state, rect) in material.iter() {
            let mut constructed = state.clone();
            toreturn.push((
                 StatetWrapper{
                    wrapped: constructed,
                    up: None,
                    down: None,
                    right: None,
                    left: None,
                },
                rect.clone()
            ));
            
        }

        return  Compounder::new(toreturn);
        
    }
}

pub struct App {
    state: TimerState
}

impl App {

    // TODO loop through the provided widgets and states paires and display thme accordingly
    /// NOTE the config parameter should become a toml table

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
            &conf.filter(&vec!["Timer"]),
            &mut terminal
        );
        
        let button = constructor::construct_button(
            &conf.filter(&vec!["Button"]),
            &mut terminal
        );

        let button_layout = button.layout.clone();
        let timer_layout = timer.layout.clone();

        if let Some(value) = comp.states.get_mut(0){

            let state = &mut value.0.wrapped;

            unsafe{

                match command {

                    Commands::Stop=>{
                        state.states.insert("working".to_string(), "false".to_string());
                    },
                    Commands::Start=>{
                        state.states.insert("working".to_string(), "true".to_string());
                    },
 //                    Commands::Revert=>{
 //                        match state.states.get("working"){
 //                            None=>{
 //                                state.states.insert("working".to_string(), "true".to_string());

 //                            }
 //                            // need to calculate the stopped duration
 //                            // and then substract from the totale duration
 //                            Some(s)=>{

 //                                let mut fi = OpenOptions::new().append(true).write(true).create(true).open("log_start").unwrap();
 //                                fi.write(format!("{:?}\n", s.eq("true")).as_bytes());

 //                                if s.eq("true") {
 //                                    let mut pduration = state.states.get("pduration").unwrap().parse::<u32>().unwrap();

 //                                    let current = SystemTime::now().duration_since(UNIX_EPOCH).expect("Could not get the current time").as_secs().to_string();
 //                                    let start = state.states.get("start").unwrap().parse::<u32>().unwrap();
 //                                    // let pduration = start.to_string();

 // 
 //                                    state.states.insert("working".to_string(), "false".to_string());
 //                                }else if s.eq("false") {
 //                                    state.states.insert("working".to_string(), "true".to_string());
 //                                }
 //                            }
 //                        }
 //                    }

                }

            }

            f.render_stateful_widget(
                timer,
                timer_layout,
                state,
            );

        }

        if let Some(value) = comp.states.get_mut(1){

            let state = &mut value.0.wrapped;

            unsafe{

                match command {

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

        // TODO should implement the start, stop functionality

        // let mut selected = comp.states.get_mut(0).unwrap().0;

        // desired behavior
        // selected = comp.states.get_mut(0).unwrap().0 
        //      this of type StatetWrapper
        //
        // selected.wrapped.insert("hovered", "false");
        // let next = selected.up
        // next.wrapped.insert("hovered", "true");
        // selected = next;

        // if let Some(value) = states.get_mut(0) {

        //     f.render_stateful_widget(
        //         timer,
        //         timer_layout,
        //         value,
        //     );

        // }

        // if let Some(value) = states.get_mut(1) {

        //     f.render_stateful_widget(
        //         button,
        //         button_layout,
        //         value,
        //     );

        // }

    }

    pub fn run<'a>(mut self) -> Result<(), Box<dyn Error>>{

        // setup terminal
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        // run the app
        let last_tick = Instant::now();

        // desired behavior
        // let app = App::new();
        // let tiemr_state: TimerState = TimerState::default();
        // app.set_state(timer_state);
        // loop {
        //     app.ui()
        //     if q is clicked {
        //         app.quit()
        //     } else if smth else is clicked {
        //        app.do_smth_else();
        //     }
        // }

        let conf = Config::read();

        let timer_conf = conf.filter(&vec!["Timer"]);
        let button_conf = conf.filter(&vec!["Button"]);

        let timer_rect = Compounder::get_rect(
            &conf, String::from("Timer"), &mut terminal
        );
        let button_rect = Compounder::get_rect(
            &conf, String::from("Button"), &mut terminal
        );

        let timer_state = construct_timer_state(&timer_conf, &mut terminal);

        let button_state = construct_button_state(&button_conf, &mut terminal);
        
        let load = vec![
            (timer_state, timer_rect),
            (button_state, button_rect)
        ];

        let mut comp = Compounder::encapsulate(load);

        loop {

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
                            App::command(command_setter::Revert);
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

        }

    }

    pub fn command(action: command_setter){

        unsafe{

            match action {

                command_setter::Start => {
                    command = Commands::Start;
                }
                command_setter::Stop=>{
                    command = Commands::Stop;
                    paused_start = Lazy::new(||{SystemTime::now().duration_since(UNIX_EPOCH).unwrap()});

                    // paused_duration += SystemTime::now().duration_since(UNIX_EPOCH).unwrap()- paused_start.clone();

                    // desired behavior
                    // on stop state {
                    //      unsafe{
                    //          paused_duration += now - paused_start 
                    //      }
                    // }
                    // paused_duration.add()
                }

                command_setter::Revert=>{

                    match command{

                        Commands::Start => {
                            command = Commands::Stop;
                        }
                        Commands::Stop=>{
                            command = Commands::Start;
                            paused_start = Lazy::new(||{SystemTime::now().duration_since(UNIX_EPOCH).unwrap()});
                            // paused_duration += SystemTime::now().duration_since(UNIX_EPOCH).unwrap() - paused_start.clone();
                        }
                        _=>{
                            panic!("commmand shoult take either Commmand::Start or Commands::Stop\n");
                        }
                        
                    }
                }
                
            }

        }

    }

    pub fn new(state: TimerState)->App{
        App { state }
    }

    pub fn quit(mut terminal:Terminal<CrosstermBackend<Stdout>>) -> Result<(), Box<dyn Error>>{

        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture,
        )?;

        terminal.show_cursor()?;

        Ok(())

    }
    
}

impl Default for App{
    fn default() -> Self {
        App {
            state: TimerState::default()
        }
    }
}
