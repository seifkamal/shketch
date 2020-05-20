use std::error;
use std::fmt;
use std::fs;
use std::io;

use termion::event::{Event, Key};
use termion::input::{MouseTerminal, TermRead};
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;

use crate::export;

#[derive(Debug)]
pub(crate) enum Error {
    NotTTY,
    IoProblem(io::Error),
    CanvasUpdateFailed(grid::Error),
    ExportFailed(export::Error),
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::NotTTY => None,
            Error::IoProblem(e) => Some(e),
            Error::CanvasUpdateFailed(e) => Some(e),
            Error::ExportFailed(e) => Some(e),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self {
            Error::NotTTY => "stream is not a TTY".to_string(),
            Error::IoProblem(e) => format!("failed to perform I/O operation; {}", e),
            Error::CanvasUpdateFailed(e) => format!("{}", e),
            Error::ExportFailed(e) => format!("{}", e),
        };

        write!(f, "Application error: {}", msg)
    }
}

impl From<io::Error> for Error {
    fn from(io_error: io::Error) -> Self {
        Error::IoProblem(io_error)
    }
}

impl From<grid::Error> for Error {
    fn from(canvas_error: grid::Error) -> Self {
        Error::CanvasUpdateFailed(canvas_error)
    }
}

impl From<export::Error> for Error {
    fn from(save_error: export::Error) -> Self {
        Error::ExportFailed(save_error)
    }
}

pub(crate) fn run() -> Result<(), Error> {
    let tty = termion::get_tty()?;
    {
        let stream = fs::File::create("/dev/stdout")?;
        if !termion::is_tty(&stream) {
            return Err(Error::NotTTY);
        }
    }

    let mut save_file_name: Option<String> = None;
    let mut canvas = {
        let screen = AlternateScreen::from(tty.try_clone()?.into_raw_mode()?);
        grid::Canvas::new(MouseTerminal::from(screen), grid::Tracer::default())
    };

    canvas.init()?;
    canvas.pin(toolbar());
    canvas.draw()?;

    for c in tty.events() {
        match c? {
            Event::Key(Key::Char('q')) => break,
            Event::Key(Key::Ctrl('s')) => {
                let blueprint: grid::Segment = canvas.snapshot().iter().sum();
                match save_file_name {
                    Some(ref name) => export::to_file_as(&blueprint, name)?,
                    None => save_file_name = Some(export::to_file(&blueprint)?),
                }
            }
            Event::Key(Key::Char('k')) => canvas.clear()?,
            Event::Key(Key::Char('u')) => canvas.undo()?,
            Event::Key(Key::Char(n)) if n.is_digit(10) => canvas.alt_style(n.into()),
            Event::Mouse(mouse_event) => canvas.update(mouse_event)?,
            _ => {}
        }

        canvas.draw()?;
    }
    Ok(())
}

fn toolbar() -> grid::Segment {
    let item = |x, y, text| grid::Segment::from_str(grid::Point::new(x, y), text);
    let mut toolbar = grid::Segment::new();
    // Actions
    toolbar += item(1, 1, "Exit (q)");
    toolbar += item(15, 1, "Clear (k)");
    toolbar += item(30, 1, "Undo (u)");
    toolbar += item(45, 1, "Save (Ctrl+s)");
    // Tools
    toolbar += item(1, 2, "Brush (1)");
    toolbar += item(15, 2, "Ruler (2)");
    toolbar
}
