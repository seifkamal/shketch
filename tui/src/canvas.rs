use std::error;
use std::fmt;
use std::io::{self, Write};
use std::result;

use crate::grid::{self, Connect, Erase};
use crate::terminal;

type Result = result::Result<(), CanvasError>;

#[derive(Debug)]
pub enum CanvasError {
    Fmt(fmt::Error),
    Io(io::Error),
}

impl error::Error for CanvasError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            CanvasError::Fmt(e) => Some(e),
            CanvasError::Io(e) => Some(e),
        }
    }
}

impl fmt::Display for CanvasError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self {
            CanvasError::Fmt(_) => "failed to format message to stream",
            CanvasError::Io(_) => "failed to perform I/O operation",
        };

        write!(f, "could not update canvas; {}", msg)
    }
}

impl From<fmt::Error> for CanvasError {
    fn from(fmt_error: fmt::Error) -> Self {
        CanvasError::Fmt(fmt_error)
    }
}

impl From<io::Error> for CanvasError {
    fn from(io_error: io::Error) -> Self {
        CanvasError::Io(io_error)
    }
}

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
    const TOOLBAR_BOUNDARY: u16 = 3;

    pub fn new(writer: W, brush: B) -> Self {
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

    pub fn update(&mut self, event: terminal::MouseEvent) -> Result {
        let terminal::MouseEvent { action, pos } = event;
        match (action, pos.x, pos.y) {
            (terminal::MouseAction::Press, x, y) => self.cursor.move_to(x, y),
            (terminal::MouseAction::Drag, x, y) => {
                // Reserve toolbar space
                if y < Self::TOOLBAR_BOUNDARY {
                    return Ok(());
                }

                match self.style {
                    Style::Plot => {
                        self.sketch += self.brush.connect(self.cursor, (x, y).into());
                        self.cursor.move_to(x, y);
                    }
                    Style::Line => {
                        self.sketch.erase(&mut self.writer)?;
                        self.sketch = self.brush.connect(self.cursor, (x, y).into());
                    }
                }
            }
            (terminal::MouseAction::Release, ..) => {
                self.base.push(self.sketch.clone());
                self.sketch.clear();
            }
        }
        Ok(())
    }

    pub fn draw(&mut self) -> Result {
        for segment in &self.base {
            write!(self.writer, "{}", segment)?;
        }
        write!(self.writer, "{}{}", self.sketch, self.overlay)?;
        self.writer.flush()?;
        Ok(())
    }

    pub fn snapshot(&self) -> Vec<grid::Segment> {
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
        write!(f, "{}{}", self.sketch, self.overlay)
    }
}
