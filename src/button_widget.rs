use std::fmt::Alignment;
use tui::{
    widgets::{Widget, Borders, BorderType},
    text::Spans,
    style::Style
};

#[derive(Debug,Clone)]
pub struct ButtonWidget<'a> {
    /// Optional title place on the upper left of the block
    pub title: Option<Spans<'a>>,
    /// Title alignment. The default is top left of the block, but one can choose to place
    /// title in the top middle, or top right of the block
    pub title_alignment: Alignment,
    /// Visible borders
    pub borders: Borders,
    /// Border style
    pub border_style: Style,
    /// Type of the border. The default is plain lines but one can choose to have rounded corners
    /// or doubled lines instead.
    pub border_type: BorderType,
    /// Widget style
    pub style: Style,
}


impl<'a> Default for ButtonWidget<'a> {
    fn default() -> Self {
        // unimplemented!();
        ButtonWidget{
            title: None,
            title_alignment: Alignment::Left,
            borders: Borders::NONE,
            border_style: Default::default(),
            border_type: BorderType::Plain,
            style: Default::default(),
        }
    }
}

// why is this implemented?
impl<'a> Widget for ButtonWidget<'a> {

    fn render(self, area: tui::layout::Rect, buf: &mut tui::buffer::Buffer) {

        if area.area() == 0 {
            return;
        }

        buf.set_style(area, self.style);

        let symbols = BorderType::line_symbols(self.border_type);

        if self.borders.intersects(Borders::LEFT) {
            for y in area.top()..area.bottom() {
                buf.get_mut(area.left(), y)
                    .set_symbol(symbols.vertical)
                    .set_style(self.border_style);
            }
        }
        if self.borders.intersects(Borders::TOP) {
            for x in area.left()..area.right() {
                buf.get_mut(x, area.top())
                    .set_symbol(symbols.horizontal)
                    .set_style(self.border_style);
            }
        }
        if self.borders.intersects(Borders::RIGHT) {
            let x = area.right() - 1;
            for y in area.top()..area.bottom() {
                buf.get_mut(x, y)
                    .set_symbol(symbols.vertical)
                    .set_style(self.border_style);
            }
        }
        if self.borders.intersects(Borders::BOTTOM) {
            let y = area.bottom() - 1;
            for x in area.left()..area.right() {
                buf.get_mut(x, y)
                    .set_symbol(symbols.horizontal)
                    .set_style(self.border_style);
            }
        }

        // Corners
        if self.borders.contains(Borders::RIGHT | Borders::BOTTOM) {
            buf.get_mut(area.right() - 1, area.bottom() - 1)
                .set_symbol(symbols.bottom_right)
                .set_style(self.border_style);
        }
        if self.borders.contains(Borders::RIGHT | Borders::TOP) {
            buf.get_mut(area.right() - 1, area.top())
                .set_symbol(symbols.top_right)
                .set_style(self.border_style);
        }
        if self.borders.contains(Borders::LEFT | Borders::BOTTOM) {
            buf.get_mut(area.left(), area.bottom() - 1)
                .set_symbol(symbols.bottom_left)
                .set_style(self.border_style);
        }
        if self.borders.contains(Borders::LEFT | Borders::TOP) {
            buf.get_mut(area.left(), area.top())
                .set_symbol(symbols.top_left)
                .set_style(self.border_style);
        }

        if let Some(title) = self.title {
            let left_border_dx = if self.borders.intersects(Borders::LEFT) {
                1
            } else {
                0
            };

            let right_border_dx = if self.borders.intersects(Borders::RIGHT) {
                1
            } else {
                0
            };

            let title_area_width = area
                .width
                .saturating_sub(left_border_dx)
                .saturating_sub(right_border_dx);

            let title_dx = match self.title_alignment {
                Alignment::Left => left_border_dx,
                Alignment::Center => area.width.saturating_sub(title.width() as u16) / 2,
                Alignment::Right => area
                    .width
                    .saturating_sub(title.width() as u16)
                    .saturating_sub(right_border_dx),
            };

            let title_x = area.left() + title_dx;
            let title_y = area.top();

            buf.set_spans(title_x, title_y, &title, title_area_width);
        }

    }

}
