use tui::{style::Style, widgets::{Borders, BorderType}};

#[derive(Debug, Clone)]
/// this widget represent a timer widget as it holds:
/// Widget Style
/// Borders
/// Borders Style
/// Borders Type
pub struct TimerWidget {
    /// Widget style
    pub style: Style,
    /// Visible borders
    pub borders: Borders,
    /// Border style
    pub border_style: Style,
    /// border type
    pub border_type: BorderType,
}

impl Default for TimerWidget{
    fn default() -> Self {
        TimerWidget {
            style: Style::default(),
            borders: Borders::ALL,
            border_style:Style::default(),
            border_type: BorderType::Rounded,
        }
    }
}
