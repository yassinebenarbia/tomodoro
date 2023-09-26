use tui::{style::Style, widgets::{Borders, BorderType, Block}};

#[derive(Debug, Clone)]
/// this widget represent a timer widget as it holds:
/// Widget Style
/// Borders
/// Borders Style
/// Borders Type
pub struct TimerWidget {
    
    /// Widget style
    pub style: Style,

    /// Optional title place on the upper left of the block
    /// Title alignment. The default is top left of the block, but one can choose to place
    /// title in the top middle, or top right of the block
    
    /// Visible borders
    pub borders: Borders,
    /// Border style
    pub border_style: Style,
    /// Type of the border. The default is plain lines but one can choose to have rounded corners
    /// or doubled lines instead.
    pub border_type: BorderType,
}

impl TimerWidget {
    pub(crate) fn default() -> TimerWidget {
        TimerWidget {
            style: Style::default(),
            borders: Borders::ALL,
            border_style:Style::default(),
            border_type: BorderType::Rounded,
        }
    }

    pub fn style(mut self, style: Style) -> TimerWidget {
        self.style = style;
        self
    }

    pub fn borders(mut self, borders: Borders) -> TimerWidget {
        self.borders = borders;
        self
    }

    pub fn border_type(mut self, border_typee: BorderType) -> TimerWidget {
        self.border_type = border_typee;
        self
    }
}
