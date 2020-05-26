use std::error;
use std::fmt;
use std::io;

use tui::{self, grid};

use crate::export;

#[derive(Debug)]
pub enum Error {
    NotTTY,
    IoProblem(io::Error),
    TerminalOperationFailed(tui::Error),
    CanvasUpdateFailed(tui::CanvasError),
    ExportFailed(export::Error),
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::NotTTY => None,
            Error::IoProblem(e) => Some(e),
            Error::TerminalOperationFailed(e) => Some(e),
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
            Error::TerminalOperationFailed(e) => format!("{}", e),
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

impl From<tui::Error> for Error {
    fn from(terminal_error: tui::Error) -> Self {
        Error::TerminalOperationFailed(terminal_error)
    }
}

impl From<tui::CanvasError> for Error {
    fn from(canvas_error: tui::CanvasError) -> Self {
        Error::CanvasUpdateFailed(canvas_error)
    }
}

impl From<export::Error> for Error {
    fn from(save_error: export::Error) -> Self {
        Error::ExportFailed(save_error)
    }
}

pub fn run() -> Result<(), Error> {
    if !tui::is_tty() {
        return Err(Error::NotTTY);
    }

    let mut terminal = tui::Terminal::default();
    terminal.wipe()?;
    terminal.enable_raw_mode();

    {
        let mut save_file_name: Option<String> = None;
        let mut canvas = tui::Canvas::new(io::stdout(), grid::Tracer::default());
        canvas.pin(toolbar());
        canvas.draw()?;

        loop {
            match terminal.read_event().unwrap() {
                tui::Event::Mouse(me) => canvas.update(me)?,
                tui::Event::Key(tui::KeyEvent { char, modifier }) => match (char, modifier) {
                    ('q', _) => break,
                    ('u', _) => canvas.undo()?,
                    ('k', _) => canvas.clear()?,
                    ('s', Some(tui::KeyModifier::Ctrl)) => {
                        let blueprint: grid::Segment = canvas.snapshot().iter().sum();
                        match save_file_name {
                            Some(ref name) => export::to_file_as(&blueprint, name)?,
                            None => save_file_name = Some(export::to_file(&blueprint)?),
                        }
                    }
                    (n, _) if n.is_digit(10) => canvas.alt_style(n.into()),
                    _ => {}
                },
            }

            canvas.draw()?;
        }
    }

    terminal.disable_raw_mode();
    terminal.restore()?;
    Ok(())
}

fn toolbar() -> grid::Segment {
    let item = |x, y, text| grid::Segment::from_str((x, y).into(), text);
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
