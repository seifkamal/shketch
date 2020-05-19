use std::convert::TryInto;
use std::error;
use std::fmt;
use std::io;

use termion::event::{Event, Key};
use termion::input::{MouseTerminal, TermRead};
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;

#[derive(Debug)]
pub(crate) enum Error {
    NotTTY,
    Io(io::Error),
    Update(design::UpdateError),
    Parse(design::ParseError),
    Save(design::SaveError),
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self {
            Error::NotTTY => "stream is not a TTY",
            Error::Io(_) => "failed to perform I/O operation",
            Error::Update(_) => "failed to update design",
            Error::Parse(_) => "failed to parse design",
            Error::Save(_) => "failed to save design",
        };

        write!(f, "Application error: {}", msg)
    }
}

impl From<design::UpdateError> for Error {
    fn from(canvas_error: design::UpdateError) -> Self {
        Error::Update(canvas_error)
    }
}

impl From<io::Error> for Error {
    fn from(io_error: io::Error) -> Self {
        Error::Io(io_error)
    }
}

impl From<design::ParseError> for Error {
    fn from(parse_error: design::ParseError) -> Self {
        Error::Parse(parse_error)
    }
}

impl From<design::SaveError> for Error {
    fn from(save_error: design::SaveError) -> Self {
        Error::Save(save_error)
    }
}

pub(crate) fn run() -> Result<(), Error> {
    let tty = termion::get_tty()?;
    {
        let stream = std::fs::File::create("/dev/stdout")?;
        if !termion::is_tty(&stream) {
            return Err(Error::NotTTY);
        }
    }

    let screen = AlternateScreen::from(tty.try_clone()?.into_raw_mode()?);
    let mut canvas = design::Canvas::new(MouseTerminal::from(screen), grid::Tracer::default());

    canvas.init()?;
    canvas.pin(toolbar());
    canvas.draw()?;

    let mut save_file_name: Option<String> = None;

    for c in tty.events() {
        match c? {
            Event::Key(Key::Char('q')) => break,
            Event::Key(Key::Ctrl('s')) => {
                let blueprint: Result<design::BluePrint, _> = canvas.snapshot().try_into();
                if let Some(name) = &save_file_name {
                    design::save_as(&blueprint?, &name)?;
                } else {
                    save_file_name = Some(design::save(&blueprint?)?);
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
    toolbar += item(1, 1, "q - Exit");
    toolbar += item(15, 1, "k - Clear");
    toolbar += item(30, 1, "u - Undo");
    toolbar += item(45, 1, "ctrl+s - Save");

    toolbar += item(1, 2, "1 - Brush");
    toolbar += item(15, 2, "2 - Ruler");
    toolbar
}
