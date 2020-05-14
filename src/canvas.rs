use std::io::Write;

use termion::clear;
use termion::cursor;
use termion::event::MouseEvent;

use crate::grid::{self, Connect};

pub enum Style {
    Plot,
    Line,
}

impl From<char> for Style {
    fn from(char: char) -> Self {
        match char {
            '2' => Style::Line,
            _ => Style::Plot,
        }
    }
}

impl Default for Style {
    fn default() -> Self {
        Style::Plot
    }
}

pub struct Canvas<W, B>
where
    W: Write,
    B: Connect,
{
    writer: W,
    brush: B,
    style: Style,
    base: Vec<grid::Segment>,
    overlay: grid::Segment,
    sketch: grid::Segment,
    cursor: grid::Point,
}

impl<W, B> Canvas<W, B>
where
    W: Write,
    B: Connect,
{
    const TOOLBAR_DIVIDER: u16 = 3;

    pub fn new(mut writer: W, brush: B) -> Self {
        write!(&mut writer, "{}{}", clear::All, cursor::Hide).unwrap();
        writer.flush().unwrap();

        Self {
            writer,
            brush,
            style: Default::default(),
            base: Default::default(),
            overlay: Default::default(),
            sketch: Default::default(),
            cursor: Default::default(),
        }
    }

    pub fn pin(&mut self, overlay: grid::Segment) {
        self.overlay = overlay;
    }

    pub fn alt_style(&mut self, style: Style) {
        self.style = style;
    }

    pub fn update(&mut self, mouse_event: MouseEvent) {
        match mouse_event {
            MouseEvent::Press(_, a, b) => self.cursor.move_to(a, b),
            MouseEvent::Hold(a, b) => {
                // Reserve toolbar space
                if b < Self::TOOLBAR_DIVIDER {
                    return;
                }

                match self.style {
                    Style::Plot => {
                        self.sketch += self.brush.connect(self.cursor, grid::Point::new(a, b));
                        self.cursor.move_to(a, b);
                    }
                    Style::Line => {
                        grid::clear_segment(self.sketch.clone(), &mut self.writer);
                        self.sketch = self.brush.connect(self.cursor, grid::Point::new(a, b));
                    }
                }
            }
            MouseEvent::Release(_, _) => {
                self.base.push(self.sketch.clone());
                self.sketch.clear();
            }
        }
    }

    pub fn draw(&mut self) {
        for segment in &self.base {
            write!(self.writer, "{}", segment).unwrap();
        }
        write!(self.writer, "{}{}", self.sketch, self.overlay).unwrap();
        self.writer.flush().unwrap();
    }

    pub fn snapshot(&self) -> Vec<grid::Segment> {
        self.base.clone()
    }

    pub fn undo(&mut self) {
        if let Some(segment) = self.base.pop() {
            grid::clear_segment(segment, &mut self.writer);
        }
    }

    pub fn clear(&mut self) {
        self.base.clear();
        self.sketch.clear();

        write!(
            self.writer,
            "{}{}",
            cursor::Goto(1, Self::TOOLBAR_DIVIDER),
            clear::All
        )
        .unwrap();
        self.writer.flush().unwrap();
    }
}

impl<W, B> Drop for Canvas<W, B>
where
    W: Write,
    B: Connect,
{
    fn drop(&mut self) {
        write!(
            self.writer,
            "{}{}{}",
            clear::All,
            cursor::Goto(1, 1),
            cursor::Show
        )
        .unwrap();
    }
}
