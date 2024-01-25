use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use std::io::Stdout;

use tui::layout::Rect;
use tui::style::Modifier;
use tui::widgets::{Borders, BorderType};
use tui::{style::Style, Terminal, backend::CrosstermBackend};

use crate::button_widget::ButtonWidget;
use crate::stateful_button::Button;
use crate::{ config::Config, state::State, statefull_timer::Timer, timer_widget::TimerWidget,};


/// Constructs the `Timer` widget based on the `Values` provided on the `values` parameter.\
/// returns a `Timer`
pub fn construct_timer(term: &mut Terminal<CrosstermBackend<Stdout>>, config: &Config) -> Timer{

    // desired timer and it's internals
    let mut toreturn = Timer::default();
    let timer = config.timer.clone();

    let mut timer_widget = TimerWidget::default();

    let style = Style::default()
        .fg(tui::style::Color::Rgb(
                timer.color.0, timer.color.1, timer.color.2
                ))
        .bg(tui::style::Color::Rgb(
                timer.background_color.0,
                timer.background_color.1,
                timer.background_color.2
                ))
        .add_modifier(Modifier::ITALIC | Modifier::BOLD);

    let term_rect = term.size().unwrap();
    let timer_rect = construct_timer_rect(config, &term_rect);

    timer_widget.style = style;
    timer_widget.borders = Borders::ALL;
    timer_widget.border_type = BorderType::Rounded;

    return toreturn
        .layout(timer_rect.x, timer_rect.y,
                timer_rect.width, timer_rect.height)
        .widget(timer_widget).to_owned();

}

/// Constructs the `StatefullButton` widget based on the `Values` provided on the `values` parameter.
/// returns a `StatefullButton`
pub fn construct_button<'b>(term: &mut Terminal<CrosstermBackend<Stdout>>, config: &Config)
    -> Button<'b>
{

    let mut toreturn = Button::default();
    let button = config.button.clone();

    let mut button_widget = ButtonWidget::default();

    let term_rec = term.size().unwrap();
    let button_rect = construct_button_rect(config, &term_rec);

    let style = Style::default()
        .fg(tui::style::Color::Rgb(
                button.color.0, button.color.1, button.color.2
                ))
        .bg(tui::style::Color::Rgb(
                button.background_color.0,
                button.background_color.1,
                button.background_color.2
                ))
        .add_modifier(Modifier::ITALIC | Modifier::BOLD);

    button_widget.style = style;

    toreturn
        .layout(button_rect.x, button_rect.y,
                button_rect.width, button_rect.height)
        .widget(button_widget);

    return toreturn;

}

/// construcuts `State` from the `values` paramater, or in another word
/// from the config file
/// This state is specifically desined for the `Timer` widget
pub fn construct_timer_state(conf:& Config, _term: &mut Terminal<CrosstermBackend<Stdout>>) -> State{

    let mut state = State::default();
    let mut timer_hashmap = HashMap::new();

    timer_hashmap.insert("displayed".to_string(), conf.timer.focus_duration.to_string());
    timer_hashmap.insert("focus_duration".to_string(), conf.timer.focus_duration.to_string());
    timer_hashmap.insert("rest_duration".to_string(), conf.timer.rest_duration.to_string());
    timer_hashmap.insert("cycles".to_string(), conf.timer.cycles.to_string());
    timer_hashmap.insert("max_cycles".to_string(), conf.timer.max_cycles.to_string());
    timer_hashmap.insert("prev_diff".to_string(), 0.to_string());
    let current = SystemTime::now().duration_since(UNIX_EPOCH).expect("Could not get the current time").as_secs().to_string();
    timer_hashmap.insert("start".to_string(), current);
    timer_hashmap.insert("working".to_string(), "true".to_string());
    timer_hashmap.insert("phase".to_string(), "focus".to_string());
    timer_hashmap.insert("focus_alarm".to_string(), conf.timer.focus_alarm.to_string());
    timer_hashmap.insert("rest_alarm".to_string(), conf.timer.rest_alarm.to_string());
    

    // TODO add support of the working state for the config file

    state.states = timer_hashmap;
    state
}

pub fn construct_button_state(conf:& Config, _term: &mut Terminal<CrosstermBackend<Stdout>>) -> State{
    let mut state = State::default();
    let mut button_hasmap = HashMap::new();

    button_hasmap.insert("clicked".to_string(), "false".to_string());
    button_hasmap.insert("hovered".to_string(), "false".to_string());

    button_hasmap.insert("focus_banner".to_string(), conf.button.focus_banner.clone());
    button_hasmap.insert("rest_banner".to_string(), conf.button.rest_banner.clone());
    button_hasmap.insert("pause_banner".to_string(), conf.button.pause_banner.clone());
    button_hasmap.insert("clickable".to_string(), "false".to_string());

    state.states = button_hasmap;

    return state;
}

pub fn construct_timer_rect(conf: &Config, terminal_rect: &Rect) -> Rect{
    let x = if conf.timer.x < 1.0 {
         (conf.timer.x * terminal_rect.width as f32) as u16
    }else{
        conf.timer.x as u16
    };

    let y = if conf.timer.y < 1.0 {
         (conf.timer.y * terminal_rect.height as f32) as u16
    }else{
        conf.timer.y as u16
    };

    let width = if conf.timer.width < 1.0 {
         (conf.timer.width * terminal_rect.width as f32) as u16
    }else{
        conf.timer.width as u16
    };

    let height = if conf.timer.height< 1.0 {
         (conf.timer.height * terminal_rect.height as f32) as u16
    }else{
        conf.timer.height as u16
    };

    return Rect{ x, y, width, height}
}

pub fn construct_button_rect(conf: &Config, terminal_rect: &Rect) -> Rect{
    let x = if conf.button.x < 1.0 {
         (conf.button.x * terminal_rect.width as f32) as u16
    }else{
        conf.button.x as u16
    };

    let y = if conf.button.y < 1.0 {
         (conf.button.y * terminal_rect.height as f32) as u16
    }else{
        conf.button.y as u16
    };

    let width = if conf.button.width < 1.0 {
         (conf.button.width * terminal_rect.width as f32) as u16
    }else{
        conf.button.width as u16
    };

    let height = if conf.button.height< 1.0 {
         (conf.button.height * terminal_rect.height as f32) as u16
    }else{
        conf.button.height as u16
    };

    return Rect{ x, y, width, height}
}


mod Test{
    
    #![allow(unused_imports)]
    use std::io::stdout;

    use crossterm::terminal;
    use tui::{Terminal, backend::CrosstermBackend};

    use crate::config::Config;

    #[test]
    fn construction() {
        let config:Config = toml::de::from_str(r#"
            [Timer]
              color = '#000000'
              width = 0.5
              height = 0.2
              x = 10
              y = 10
            [Button]
              color = '#000000'
              width = 0.5
              height = 0.3
              x = 20
              y = 10
        "#).unwrap();

        todo!();
    }

}
