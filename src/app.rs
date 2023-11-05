use std::{time::{Duration, Instant}, error::Error, fmt::{Alignment, Debug}, any::{self, Any}, io::Stdout, default};
use std::io;
use crossterm::{terminal::{enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen, DisableLineWrap}, execute, event::{EnableMouseCapture, DisableMouseCapture, KeyCode, Event, self}, cursor::MoveUp};
use json::JsonValue;
use tui::{
    backend::{Backend, CrosstermBackend},
    widgets::{Borders, BorderType, Block, StatefulWidget},
    layout::Rect,
    Frame, style::{Style, Color, Modifier}, buffer::Buffer, Terminal,
};

use crate::{
    stateful_button::{StatefullButton, ButtonState}
    ,button::Button, button_widget::ButtonWidget, statefull_timer::Timer,
    timer_widget::TimerWidget, timer_state::TimerState, widget_fixer::Fixer,
    displayable::Displayable, screen::{Screen, self}, config::Config, directions::Directions
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

    type State = u16;
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
}

pub struct App {
    state: TimerState
}

impl App {

    // TODO: set the ui such that it can have mutable and immutable reference to f simultaniously,
    // if possible
    pub fn ui<'a ,B: Backend>(f: & mut Frame<'a ,B>, timerstate: &mut TimerState) {

        let mut fixer = Fixer::new(f);

        let button2 = Button::default()
            .widget(
                Color::LightRed, Color::LightMagenta, Modifier::BOLD,
                String::from("stuff"), Borders::ALL
            ).layout(1, 1, 10, 10);

        // f.render_widget(button2.get_widget(), button2.get_layout());

        // unimplemented!
        let mut onhover = |rect: Rect, buf:&mut Buffer, st:&mut ButtonState|{};
        let mut onclick= |rect: Rect, buf:&mut Buffer, st:&mut ButtonState|{};

        let button: StatefullButton = StatefullButton::default()
            .layout(fixer.xratio(40), fixer.yratio(40), fixer.wratio(10), fixer.hratio(10))
            .widget(
                ButtonWidget::default()
                    .style(
                        Style::default()
                            .fg(Color::Red)
                            .bg(Color::Indexed(19))
                    )
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .title("some title".to_string())
            )
            .onhover(&mut onhover)
            .onclick(&mut onclick);

        let layout = button.get_layout().clone();

        let mut state:ButtonState = ButtonState::new(true, true);



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

        let timer:Timer = Timer::default()
            .layout(fixer.xratio(40), fixer.yratio(30), fixer.wratio(10), fixer.hratio(10))
            .widget(
                TimerWidget::default()
                    .style(
                        Style::default()
                            .fg(Color::Yellow)
                            .bg(Color::Red)
                    )
                    .borders(Borders::ALL)
                    .border_type(BorderType::Double)
            )
            .time(Duration::from_secs(1501));

        // desired behavior
        // .on_clock_tick() // here the closure should take 
        // an instance of self, the Rectangle, the Buffer, and the BufferState respectively
        // where as this will run every second
        // .on_idle_state() // same goes for this
        // this will run whenever the timer reaches 0
        // NOTE: This conditions where should the closures run, is checked on the mail loop

        let timer_layout = timer.layout.clone();


        // f.render_widget(get_block(String::from("hello")), Rect::new(10, 10, 5, 5));
        // f.render_widget(Timer, Timer_area);
        // f.render_widget(Wall, Wall_area);
        // f.render_widget(Button, Button_area);
        let cycles_display = Block::default()
            .title(timerstate.get_cycle().to_string())
            .style(
                Style::default()
                    .fg(Color::Yellow)
                    .bg(Color::Red)
            );

        f.render_widget(cycles_display, Rect::new(fixer.xratio(90) , fixer.yratio(1), fixer.wratio(5) , fixer.hratio(3)));

        f.render_stateful_widget(
            button,
            layout,
            &mut state
        );

        f.render_stateful_widget(
            timer,
            timer_layout,
            timerstate,
        );


    }

    /// NOTE the config parameter should become a JsonValue
    pub fn renderui<'a, B>(f: & mut Frame<'a ,B>, config:&mut Config, timerstate: &mut TimerState) where
        B: Backend,
    {

        let widgets_name_list = vec!["Timer", "Button", "Counter"];

        let filtered_conf = config.filter(widgets_name_list);

        let default = config.filter(vec!["default"]);

        let mut fixer = Fixer::new(f);

        // unimplemented!
        let mut onhover = |rect: Rect, buf:&mut Buffer, st:&mut ButtonState|{};
        let mut onclick = |rect: Rect, buf:&mut Buffer, st:&mut ButtonState|{};

        let button: StatefullButton = StatefullButton::default()
            .layout(fixer.xratio(40), fixer.yratio(40), fixer.wratio(10), fixer.hratio(10))
            .widget(
                ButtonWidget::default()
                    .style(
                        Style::default()
                            .fg(Color::Red)
                            .bg(Color::Indexed(19))
                    )
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .title("some title".to_string())
            )
            .onhover(&mut onhover)
            .onclick(&mut onclick);

        let layout = button.get_layout().clone();

        let mut state:ButtonState = ButtonState::new(true, true);

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

        let timer:Timer = Timer::default()
            .layout(fixer.xratio(40), fixer.yratio(30), fixer.wratio(10), fixer.hratio(10))
            .widget(
                TimerWidget::default()
                    .style(
                        Style::default()
                            .fg(Color::Yellow)
                            .bg(Color::Red)
                    )
                    .borders(Borders::ALL)
                    .border_type(BorderType::Double)
            );

        // desired behavior
        // .on_clock_tick() // here the closure should take 
        // an instance of self, the Rectangle, the Buffer, and the BufferState respectively
        // where as this will run every second
        // .on_idle_state() // same goes for this
        // this will run whenever the timer reaches 0
        // NOTE: This conditions where should the closures run, is checked on the mail loop

        let timer_layout = timer.layout.clone();


        // f.render_widget(get_block(String::from("hello")), Rect::new(10, 10, 5, 5));
        // f.render_widget(Timer, Timer_area);
        // f.render_widget(Wall, Wall_area);
        // f.render_widget(Button, Button_area);
        let cycles_display = Block::default()
            .title(timerstate.get_cycle().to_string())
            .style(
                Style::default()
                    .fg(Color::Yellow)
                    .bg(Color::Red)
            );

        f.render_widget(
            cycles_display, 
            Rect::new(fixer.xratio(90) , fixer.yratio(1), fixer.wratio(5) , fixer.hratio(3))
        );

        f.render_stateful_widget(
            button,
            layout,
            &mut state
        );

        f.render_stateful_widget(
            timer,
            timer_layout,
            timerstate,
        );

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

        // TODO: should add a new countdown stopwatch and the couter should
        // be independed from each others
        let mut timerstate: TimerState = TimerState::default()
            .duration(Duration::from_secs(1500))
            .displayed(Duration::from_secs(1500));

        // desired behavior
        // let app = App::create(config_path);
        // loop {
        //  app.draw();
        // }
        let dumy = Dumy::new(1, 1);

        let timer: Timer = Timer::default();

        let mut conf = Config::read();

        loop {

            terminal.draw(|f| {

                App::renderui::<CrosstermBackend<Stdout>>(f, &mut conf, &mut timerstate);

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
