use std::{
    fmt::Debug, io, process::Command, thread, time::{Duration, SystemTime, UNIX_EPOCH}
};

use tui::{
    backend::CrosstermBackend,
    layout::Rect,
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{BorderType, Borders, StatefulWidget},
    Terminal,
};
// use simulate_input::{Key, KeyCode};

use crate::{
    app::{CYCLES, PAUSED_DURATION, PAUSED_START_TIME, PHASE, QUIT, SMALL_PAUSED_DURATION},
    capabilities::{compare_rect, time_conversion},
    displayable::Displayable,
    player::Player,
    state,
    timer_widget::TimerWidget,
};

/// This shall represent a Timer, as with the timer (TimerWidget),
/// frame (rectangel), layout (rectangel) and time (duration)
#[derive(Clone, Debug)]
pub struct Timer {
    /// The Counting duration
    pub time: Duration,
    /// Frame, the acrea which this widget is enveloped on
    pub frame: Rect,
    /// The area in which the timer is displayed
    pub layout: Rect,
    /// Timer widget, related to what is displayed and it's style
    /// Holds style, borders, border_style and border_type
    pub widget: TimerWidget,
}

impl Default for Timer {
    fn default() -> Self {
        let backend = CrosstermBackend::new(io::stdout());
        let terminal = Terminal::new(backend).unwrap();
        Timer {
            time: Duration::from_secs(1500),
            layout: Rect::new(1, 1, 1, 1),
            widget: TimerWidget::default(),
            frame: terminal.size().unwrap(),
        }
    }
}

impl StatefulWidget for Timer {
    type State = state::State;
    fn render(self, area: Rect, buf: &mut tui::buffer::Buffer, state: &mut Self::State) {
        // checks the steate,
        // if the displayed time is eqal to the sate time
        // 1) substract time_step from the displayed time
        // 2) render the widget
        // if the displayed time is not equal to zero
        // 1) substract time_step from the displayed time
        // 2) render the widget
        if area.area() == 0 {
            return;
        }

        buf.set_style(area, self.widget.style);
        let symbols = BorderType::line_symbols(self.widget.border_type);
        // sides
        if self.widget.borders.intersects(Borders::LEFT) {
            for y in area.top()..area.bottom() {
                buf.get_mut(area.left(), y)
                    .set_symbol(symbols.vertical)
                    .set_style(self.widget.border_style);
            }
        }
        if self.widget.borders.intersects(Borders::TOP) {
            for x in area.left()..area.right() {
                buf.get_mut(x, area.top())
                    .set_symbol(symbols.horizontal)
                    .set_style(self.widget.border_style);
            }
        }
        if self.widget.borders.intersects(Borders::RIGHT) {
            let x = area.right() - 1;
            for y in area.top()..area.bottom() {
                buf.get_mut(x, y)
                    .set_symbol(symbols.vertical)
                    .set_style(self.widget.border_style);
            }
        }
        if self.widget.borders.intersects(Borders::BOTTOM) {
            let y = area.bottom() - 1;
            for x in area.left()..area.right() {
                buf.get_mut(x, y)
                    .set_symbol(symbols.horizontal)
                    .set_style(self.widget.border_style);
            }
        }

        // corners
        if self
            .widget
            .borders
            .contains(Borders::RIGHT | Borders::BOTTOM)
        {
            buf.get_mut(area.right() - 1, area.bottom() - 1)
                .set_symbol(symbols.bottom_right)
                .set_style(self.widget.border_style);
        }
        if self.widget.borders.contains(Borders::RIGHT | Borders::TOP) {
            buf.get_mut(area.right() - 1, area.top())
                .set_symbol(symbols.top_right)
                .set_style(self.widget.border_style);
        }
        if self
            .widget
            .borders
            .contains(Borders::LEFT | Borders::BOTTOM)
        {
            buf.get_mut(area.left(), area.bottom() - 1)
                .set_symbol(symbols.bottom_left)
                .set_style(self.widget.border_style);
        }
        if self.widget.borders.contains(Borders::LEFT | Borders::TOP) {
            buf.get_mut(area.left(), area.top())
                .set_symbol(symbols.top_left)
                .set_style(self.widget.border_style);
        }

        // duration as a string
        let s_displayed = state.states.get("displayed").unwrap();
        // converting the duration from a string to a u64 to a Duration
        let displayed_duration = Duration::from_secs(s_displayed.parse::<u64>().unwrap());

        //TODO: move this call out of the rendering loop
        let time = time_conversion(displayed_duration);

        let time = Spans::from(vec![Span::styled(time, Style::default().fg(Color::Yellow))]);

        // time
        let left_border_dx = if self.widget.borders.intersects(Borders::LEFT) {
            1
        } else {
            0
        };

        let right_border_dx = if self.widget.borders.intersects(Borders::RIGHT) {
            1
        } else {
            0
        };

        let time_area_width = area
            .width
            .saturating_sub(left_border_dx)
            .saturating_sub(right_border_dx);

        let time_dx = area.width.saturating_sub(time.width() as u16) / 2;
        let time_dy = area.height / 2;

        let time_x = area.left() + time_dx;
        let time_y = area.top() + time_dy;

        buf.set_spans(time_x, time_y, &time, time_area_width);

        unsafe {
            // the duration from which the application started
            let start = PAUSED_START_TIME.clone();

            // checking the working state of the timer, it's either
            // "true" for incrementing the time and "false" for not
            if let Some(s) = state.states.get("working") {
                // meaning that the timer should be working
                if s == "true" {
                    self.manage_state(state);
                    SMALL_PAUSED_DURATION = Duration::ZERO;
                // meaning that the timer should NOT be working
                } else {
                    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
                    SMALL_PAUSED_DURATION = now - start;
                    // to know how long did we spend on posing
                }
            }
        }
    }
}

/// TODO undoo this
impl Displayable for Timer {
    fn manage_state(&self, state: &mut state::State) {
        unsafe {
            // the phase of the timer, "focus" or "rest"
            let phase = state
                .states
                .get("phase")
                .expect("unable to locate the focus_banner in the button_state");

            // the cyelces that the timer has gone through
            // intital value is 0
            let cycles = state
                .states
                .get("cycles")
                .expect("unable to get the cycle number from the timer state")
                .parse::<u64>()
                .unwrap();

            let max_cycles = state
                .get_states()
                .get("max_cycles")
                .expect("unable to get max_cycles from timer state");

            // if the max_cycles is declared, and the current cycle count is equal to the
            // maximum amount of cycles, we should quit the program
            if max_cycles != "inf" && max_cycles.parse::<u64>().unwrap() <= cycles {
                QUIT = true;
                let focus_path = state
                    .get_states()
                    .get("focus_alarm")
                    .expect("unable to locate te focus_alarm path")
                    .clone();
                if focus_path != "" {
                    thread::spawn(move || {
                        if focus_path != "" {
                            let mut full_focus_path = std::env::var("TOMODORO_PATH").unwrap();
                            full_focus_path.push_str("/");
                            full_focus_path.push_str(focus_path.as_str());
                            let player = Player::new(&full_focus_path);
                            player.play_until(Duration::from_secs(2));
                        }
                    })
                    .join()
                        .unwrap();
                    }
            }

            // the systime of which the application started
            let start = state
                .get_states()
                .get("start")
                .expect("no start time is not provided");

            // duration from which the application started
            let start_duration = UNIX_EPOCH + Duration::from_secs(start.parse::<u64>().unwrap());

            let diff = SystemTime::now()
                // the displayed duration is the worked time plus the paused time
                .duration_since(start_duration + SMALL_PAUSED_DURATION + PAUSED_DURATION)
                .expect("unable to manage time")
                .as_secs();

            // meaning that time has advanced since the beginning of the counter
            // need to do a modulo opperation to get the number of cycles
            if diff > 0 {
                // previously calcuated time difference. (previous clock tick)
                let prev_diff = state
                    .get_states()
                    .get("prev_diff")
                    .expect("couldn't load the prev_diff state of the Timer")
                    .parse::<u64>()
                    .unwrap();

                // the length/duration of the focu phase
                let focus_duration = state
                    .get_states()
                    .get("focus_duration")
                    .expect("couldn't load the focus_duration state of Timer")
                    .parse::<u64>()
                    .unwrap();

                // the duration of the rest phase
                let rest_duration = state
                    .get_states()
                    .get("rest_duration")
                    .expect("couldn't load the rest_duration state of the Timer")
                    .parse::<u64>()
                    .unwrap();

                // time has advanced
                if prev_diff < diff {
                    // focus phase
                    if phase == "focus" {
                        // how many seconds have passed in this phase
                        let ndiff = (diff - CYCLES * rest_duration) % focus_duration;
                        let displayed = (Duration::from_secs(focus_duration)
                            - Duration::from_secs(ndiff))
                        .as_secs()
                        .to_string();

                        state.states.insert("displayed".to_string(), displayed);
                        // end of focus phase
                        if ndiff == focus_duration - 1 {
                            state
                                .states
                                .insert(String::from("prev_diff"), diff.to_string());
                            let focus_path = state
                                .get_states()
                                .get("focus_alarm")
                                .expect("unable to locate te focus_alarm path")
                                .clone();

                            if focus_path != "" {
                                thread::spawn(move || {
                                    if focus_path != "" {
                                        let mut full_focus_path =
                                            std::env::var("TOMODORO_PATH").unwrap();
                                        full_focus_path.push_str("/");
                                        full_focus_path.push_str(focus_path.as_str());
                                        let player = Player::new(&full_focus_path);
                                        player.play_until(Duration::from_secs(2));
                                    }
                                });
                            }
                            CYCLES += 1;
                            state
                                .states
                                .insert("cycles".to_string(), (cycles + CYCLES).to_string());
                            state.states.insert("phase".to_string(), "rest".to_string());
                            //  depending on the config settings, we are running a routine at the end of the focus phase
                            if state.states.get("hook.enable").unwrap_or(&String::from("false")).eq(&String::from("true")) &&
                                state.states.get("hook.focus.enable").unwrap_or(&String::from("false")).eq(&String::from("true")) && 
                                state.states.get("hook.focus.after").unwrap_or(&String::from("start")).eq(&String::from("End"))
                            {
                                Command::new("bash")
                                    .arg(state.states.get("hook.focus.path").unwrap_or(&String::new()))
                                    .output()
                                    .map_err(|err| {
                                        format!("Error: couldn't run focus routine at the End phase!\n{}", err)
                                    }).unwrap();
                            }
                            PHASE = "rest";
                            // the start of a focus phase
                        } else if ndiff == 1 {
                            //  we are running a routine at the start of the focus phase
                            if state.states.get("hook.enable").unwrap_or(&String::from("false")).eq(&String::from("true")) &&
                                state.states.get("hook.focus.enable").unwrap_or(&String::from("false")).eq(&String::from("true")) && 
                                state.states.get("hook.focus.after").unwrap_or(&String::from("start")).eq(&String::from("Start"))
                            {
                                Command::new("bash")
                                    .arg(state.states.get("hook.focus.path").unwrap_or(&String::new()))
                                    .output()
                                    .map_err(|err| {
                                        format!("Error: couldn't run focus routine at the Start phase!\n{}", err)
                                    }).unwrap();
                            }
                        }
                    } else if phase == "rest" {
                        let ndiff = (diff - CYCLES * focus_duration) % rest_duration;
                        let displayed = (Duration::from_secs(rest_duration)
                            - Duration::from_secs(ndiff))
                        .as_secs()
                        .to_string();
                        state.states.insert("displayed".to_string(), displayed);
                        // end of rest phase
                        if ndiff == rest_duration - 1 {
                            state
                                .states
                                .insert(String::from("prev_diff"), diff.to_string());
                            state
                                .states
                                .insert("phase".to_string(), "focus".to_string());
                            PHASE = "focus";
                            let rest_path = state
                                .get_states()
                                .get("rest_alarm")
                                .expect("unable to locate te rest_alarm path")
                                .clone();
                            if rest_path != "" {
                                thread::spawn(move || {
                                    if rest_path != "" {
                                        let mut full_rest_path =
                                            std::env::var("TOMODORO_PATH").unwrap();
                                        full_rest_path.push_str("/");
                                        full_rest_path.push_str(rest_path.as_str());
                                        let player = Player::new(&full_rest_path);
                                        player.play_until(Duration::from_secs(2));
                                    }
                                });
                            }
                            // depending on the config settings, we are running a routine at the end of the focus phase
                            if state.states.get("hook.enable").unwrap_or(&String::from("false")).eq(&String::from("true")) &&
                                state.states.get("hook.rest.enable").unwrap_or(&String::from("false")).eq(&String::from("true")) && 
                                state.states.get("hook.rest.after").unwrap_or(&String::from("start")).eq(&String::from("End")) 
                            {

                                Command::new("bash")
                                    .arg(state.states.get("hook.rest.path").unwrap_or(&String::new()))
                                    .output()
                                    .map_err(|err| {
                                        format!("Error: couldn't run rest routine at the End phase!\n{}", err)
                                    }).unwrap();
                            }
                            //  start of a rest phase
                        }else if ndiff == 1 {
                            //  we are running a routine at the start of the rest phase
                            if state.states.get("hook.enable").unwrap_or(&String::from("false")).eq(&String::from("true")) &&
                                state.states.get("hook.rest.enable").unwrap_or(&String::from("false")).eq(&String::from("true")) && 
                                state.states.get("hook.rest.after").unwrap_or(&String::from("start")).eq(&String::from("Start"))
                            {
                                Command::new("bash")
                                    .arg(state.states.get("hook.rest.path").unwrap_or(&String::new()))
                                    .output()
                                    .map_err(|err| {
                                        format!("Error: couldn't run focus routine at the Start phase!\n{}", err)
                                    }).unwrap();
                            }
                        }
                    }

                    // if the time advanced to the point where there is a whole 1s diffrence between
                    // the current and previously calculated difference
                    state
                        .states
                        .insert("prev_diff".to_string(), diff.to_string());
                }
            }
        }
    }

    fn layout(&self) -> Rect {
        self.layout.clone()
    }
}

impl Timer {
    pub fn layout(&mut self, x: u16, y: u16, width: u16, height: u16) -> &mut Self {
        let layout = Rect::new(x, y, width, height);

        // the frame determine the space the widget "claims" to be it's contour
        match compare_rect(&self.frame, &layout) {
            Ok(_) => {
                self.layout = layout;
                self
            }
            Err(msg) => {
                panic!("following erro occured with widget{:?}\n{}", layout, msg)
            }
        }
    }

    pub fn widget(&mut self, widget: TimerWidget) -> &mut Self {
        self.widget = widget;
        self
    }
}
