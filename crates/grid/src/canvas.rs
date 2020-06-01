use std::error;
use std::fmt;
use std::io::Write;
use std::result;

use crate::path::{self, Connect};
use crate::unit::{self, Erase};

type Result<T = ()> = result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Style {
    Plot,
    Line,
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
    base: Vec<unit::Segment>,
    sketch: unit::Segment,
    cursor: path::Point,
}

impl<W, B> Canvas<W, B>
where
    W: Write,
    B: Connect,
{
    pub fn new(writer: W, brush: B) -> Self {
        Self {
            writer,
            brush,
            style: Default::default(),
            base: Default::default(),
            sketch: Default::default(),
            cursor: Default::default(),
        }
    }

    pub fn alt_style(&mut self, style: Style) {
        self.style = style;
    }

    pub fn update(&mut self, event: terminal::MouseEvent) -> Result {
        match (event.action, event.pos) {
            (terminal::MouseAction::Press, (x, y)) => self.cursor.move_to(x, y),
            (terminal::MouseAction::Drag, (x, y)) => match self.style {
                Style::Plot => {
                    self.sketch += self.brush.connect(self.cursor, (x, y).into());
                    self.cursor.move_to(x, y);
                }
                Style::Line => {
                    self.sketch.erase(&mut self.writer)?;
                    self.sketch = self.brush.connect(self.cursor, (x, y).into());
                }
            },
            (terminal::MouseAction::Release, _) => {
                self.base.push(self.sketch.clone());
                self.sketch.clear();
            }
        }
        Ok(())
    }

    pub fn snapshot(&self) -> Vec<unit::Segment> {
        self.base.clone()
    }

    pub fn undo(&mut self) -> Result {
        if let Some(mut segment) = self.base.pop() {
            segment.erase(&mut self.writer)?;
        }
        Ok(())
    }

    pub fn clear(&mut self) -> Result {
        for segment in &mut self.base {
            segment.erase(&mut self.writer)?;
        }
        self.base.clear();
        self.sketch.erase(&mut self.writer)?;
        self.sketch.clear();
        Ok(())
    }
}

impl<W, B> fmt::Display for Canvas<W, B>
where
    W: Write,
    B: Connect,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for segment in &self.base {
            write!(f, "{}", segment)?;
        }
        write!(f, "{}", self.sketch)
    }
}
