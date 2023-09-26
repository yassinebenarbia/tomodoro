pub struct runner;

// NOTE: this will come in handy later, will use the runner to run the application 
// instad of the ui, si i can implement the following:
// App::new();
// if condition {
//     App.do_thing();
// } else {
//     App.do_another
// }

impl runner{

    pub fn run() -> Result<(), Box<dyn Error>> {

        // setup terminal
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        // run the app
        let last_tick = Instant::now();

        loop {

            terminal.draw(|f| {

                ui(f);

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

}
