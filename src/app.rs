use std::{time::{Duration, Instant}, io, error::Error};
use crossterm::{terminal::{enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}, execute, event::{EnableMouseCapture, DisableMouseCapture, KeyCode, Event, self}};
use tui::{
    backend::{Backend, CrosstermBackend},
    widgets::{Borders, BorderType, Block},
    layout::Rect,
    Frame, style::{Style, Color, Modifier}, buffer::Buffer, Terminal,
};
use crate::{
    stateful_button::{StatefullButton, ButtonState}
    , button::Button, button_widget::ButtonWidget, statefull_timer::Timer, timer_widget::TimerWidget, timer_state::TimerState
};

/// widget
fn get_block<'a>(title: String) -> ButtonWidget<'a>{
    return ButtonWidget::default()
        .style(
            Style::default()
            .fg(Color::Red).bg(Color::Cyan)
            .add_modifier(Modifier::BOLD | Modifier::ITALIC)
        )
        .title(title.clone())
        .borders(Borders::ALL);
}

pub struct App {
    state: TimerState
}

impl App {

    pub fn ui<B: Backend>(f: &mut Frame<B>, timerstate: &mut TimerState) {

        let mut button2 = Button::default()
            .widget(
                Color::LightRed, Color::LightMagenta, Modifier::BOLD,
                String::from("stuff"), Borders::ALL
            ).layout(1, 1, 10, 10);

        // f.render_widget(button2.get_widget(), button2.get_layout());

        // unimplemented!
        let mut onhover = |rect: Rect, buf:&mut Buffer, st:&mut ButtonState|{};
        let mut onclick= |rect: Rect, buf:&mut Buffer, st:&mut ButtonState|{};

        let button: StatefullButton = StatefullButton::default()
            .layout(70, 15, 40, 7)
            .widget(
                ButtonWidget::default()
                    .style(
                        Style::default()
                            .fg(Color::Red)
                            .bg(Color::Indexed(19))
                    )
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
            )
            .onhover(&mut onhover)
            .onclick(&mut onclick);

        let layout = button.get_layout().clone();

        let mut state:ButtonState = ButtonState::new(true, true);

        f.render_stateful_widget(
            button,
            layout,
            &mut state
        );

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

        Block::default().title("hello");
        let timer:Timer = Timer::default()
            .layout(10, 10, 40, 7)
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
            .time(Duration::from_secs(10));

        let timer_layout = timer.layout.clone();

        f.render_stateful_widget(
            timer,
            timer_layout,
            timerstate,
        );

        // f.render_widget(get_block(String::from("hello")), Rect::new(10, 10, 5, 5));
        // f.render_widget(Timer, Timer_area);
        // f.render_widget(Wall, Wall_area);
        // f.render_widget(Button, Button_area);
    }

    pub fn run(mut self) -> Result<(), Box<dyn Error>>{

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

        let mut timerstate: TimerState = TimerState::default();
        loop {

            terminal.draw(|f| {

                App::ui(f, &mut timerstate);

            })?;

            let timeout = Duration::from_millis(250)
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if crossterm::event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char('q') =>{

                            execute!(
                                terminal.backend_mut(),
                                LeaveAlternateScreen,
                                DisableMouseCapture
                            )?;
                            terminal.show_cursor()?;

                            return Ok(())
                        },
                        _ => {}
                    }
                }
            }
        }
    }

    pub fn new(state: TimerState)->App{
        App { state: state }
    }

    pub fn quit(mut self){

    }

    pub fn set_state(mut self)->App{
        self
    }
    
}

impl Default for App{
    fn default() -> Self {
        App {
            state: TimerState::default()
        }
    }
}
