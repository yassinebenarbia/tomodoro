use std::{time::{Duration, Instant, SystemTime, UNIX_EPOCH}, error::Error, fmt::{Alignment, Debug, Formatter}, any::{self, Any}, io::Stdout, default, cmp::Ordering, collections::HashMap, ops::Deref};
use std::io;
use crossterm::{terminal::{enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen, DisableLineWrap}, execute, event::{EnableMouseCapture, DisableMouseCapture, KeyCode, Event, self}, cursor::MoveUp};
use json::JsonValue;
use tui::{
    backend::{Backend, CrosstermBackend},
    widgets::{Borders, BorderType, Block, StatefulWidget},
    layout::Rect,
    Frame, style::{Style, Color, Modifier}, buffer::Buffer, Terminal, terminal,
};

use crate::{
    stateful_button::{Button, ButtonState}
    ,button::Cadre, button_widget::ButtonWidget, statefull_timer::Timer,
    timer_widget::TimerWidget, timer_state::TimerState, widget_fixer::Fixer,
    displayable::Displayable, screen::{Screen, self, Compounder}, config::Config, directions::Directions, constructor::{ truck, self, construct_timer_state, construct_button_state}, State, state, trait_holder::TraitHolder
};

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

#[derive(Debug, Clone)]
pub struct Dumy{
    x: u16,
    y: u16,
}

impl Dumy {
    pub fn new(x: u16, y: u16) -> Dumy {
        Dumy { x, y }
    }
}

impl StatefulWidget for Dumy {

    type State = State::State;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        
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

    fn highlight(&self) {
        
    }
    fn manage_state(&self, state: &mut crate::State::State) {
        
    }
    fn layout(&self)->Rect {
        Rect { x: self.x, y: self.y, width: 0, height: 0 }
    }
}

pub struct App {
    state: TimerState
}

impl App {

    // TODO loop through the provided widgets and states paires and display thme accordingly
    /// NOTE the config parameter should become a JsonValue

    pub fn renderui<'a, B>(
        f: & mut Frame<'a ,B>,
        states: &mut Vec<State::State>,
        conf:&Config
    ) where
        B: Backend,
    {
        let stdout = io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend).unwrap();

        let mut fixer = Fixer::new(f);

        // unimplemented!
        let mut onhover = |rect: Rect, buf:&mut Buffer, st:&mut State::State|{};
        let mut onclick = |rect: Rect, buf:&mut Buffer, st:&mut State::State|{};

        // // desired behavior
        // // let app = App::new();
        // // let tiemr_state: TimerState = TimerState::default();
        // // app.set_state(timer_state);
        // // loop {
        // //     app.run()
        // //     if q is clicked {
        // //         app.quit()
        // //     } else if smth else is clicked {
        // //        app.do_smth_else();
        // //     }
        // // }

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


        if let Some(value) = states.get_mut(0) {

            f.render_stateful_widget(
                timer,
                timer_layout,
                value,
            );

        }

        if let Some(value) = states.get_mut(1) {

            f.render_stateful_widget(
                button,
                button_layout,
                value,
            );

        }

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
        // let compounder = Compounder::new(vec![(timer_conf, )])

        let mut states = vec![
            construct_timer_state(&timer_conf, &mut terminal),
            construct_button_state(&button_conf, &mut terminal)
        ];

        loop {

            terminal.draw(|f| {

                App::renderui::<CrosstermBackend<Stdout>>(f,
                    &mut states,
                    &conf,
                );

            })?;

            let timeout = Duration::from_millis(250)
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if crossterm::event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char('q') =>{
                            return App::quit(terminal)
                        },
                        KeyCode::Char('j') =>{
                            App::select(Directions::Down)
                        },
                        KeyCode::Char('k') =>{
                            App::select(Directions::Up)
                        },
                        KeyCode::Char('l') =>{
                            App::select(Directions::Left)
                        },
                        KeyCode::Char('h') =>{
                            App::select(Directions::Righ)
                        },
                        _ => {}
                    }
                }
            }
        }

    }

    pub fn select(direction: Directions){

        match direction {

            Directions::Up =>{

                todo!("this should select the up widget");

            },
            Directions::Down => {

                todo!("this should select the left widget");

            },
            Directions::Righ => {

                todo!("this should select the right widget");

            },
            Directions::Left =>{

                todo!("this should select the left widget");

            }

        }

    }

    pub fn new(state: TimerState)->App{
        App { state: state }
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
