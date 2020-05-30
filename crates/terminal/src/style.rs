use std::fmt;

use crossterm::style;

pub type Color = style::Color;

#[rustfmt::skip]
pub static RESET_COLOR: Format = Format {
    bg_color: Color::Reset,
    fg_color: Color::Reset,
};

#[derive(Debug, Copy, Clone)]
pub struct Format {
    pub bg_color: Color,
    pub fg_color: Color,
}

impl Format {
    pub fn new(bg_color: Color, fg_color: Color) -> Self {
        Self { bg_color, fg_color }
    }
}

impl Default for Format {
    fn default() -> Self {
        RESET_COLOR
    }
}

impl fmt::Display for Format {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        crossterm::queue!(
            f,
            style::SetBackgroundColor(self.bg_color),
            style::SetForegroundColor(self.fg_color),
        )
        .map_err(|_| fmt::Error)?;
        Ok(())
    }
}
