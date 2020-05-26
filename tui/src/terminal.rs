use std::convert::{TryFrom, TryInto};
use std::error;
use std::fmt;
use std::io;
use std::result;

use crossterm::terminal;
use crossterm::tty::IsTty;
use crossterm::ExecutableCommand;
use crossterm::{event, ErrorKind};

use crate::grid;

pub fn is_tty() -> bool {
    io::stdout().is_tty() && io::stdin().is_tty()
}

type Result = result::Result<(), Error>;

#[derive(Debug)]
pub struct Error(crossterm::ErrorKind);

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<crossterm::ErrorKind> for Error {
    fn from(error: ErrorKind) -> Self {
        Self(error)
    }
}

pub struct Terminal {
    stdout: io::Stdout,
}

impl Terminal {
    pub fn new(stdout: io::Stdout) -> Self {
        Self { stdout }
    }

    pub fn wipe(&mut self) -> Result {
        self.stdout.execute(terminal::EnterAlternateScreen)?;
        self.stdout.execute(crossterm::cursor::Hide)?;
        self.stdout.execute(event::EnableMouseCapture)?;
        self.clear()?;
        Ok(())
    }

    pub fn restore(&mut self) -> Result {
        self.clear()?;
        self.stdout.execute(event::DisableMouseCapture)?;
        self.stdout.execute(crossterm::cursor::Show)?;
        self.stdout.execute(terminal::LeaveAlternateScreen)?;
        Ok(())
    }

    pub fn clear(&mut self) -> Result {
        self.stdout.execute(terminal::Clear(terminal::ClearType::All))?;
        Ok(())
    }

    pub fn enable_raw_mode(&self) {
        terminal::enable_raw_mode().unwrap();
    }

    pub fn disable_raw_mode(&self) {
        terminal::disable_raw_mode().unwrap();
    }

    pub fn read_event(&self) -> result::Result<Event, InputError> {
        event::read().unwrap().try_into()
    }
}

impl Default for Terminal {
    fn default() -> Self {
        Self::new(io::stdout())
    }
}

#[derive(Debug)]
pub enum InputError {
    UnsupportedEvent,
    UnsupportedKeyEvent,
    UnsupportedMouseEvent,
}

impl error::Error for InputError {}

impl fmt::Display for InputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InputError::UnsupportedEvent => write!(f, "unsupported input event"),
            InputError::UnsupportedKeyEvent => write!(f, "unsupported input key event"),
            InputError::UnsupportedMouseEvent => write!(f, "unsupported input mouse event"),
        }
    }
}

pub enum Event {
    Key(KeyEvent),
    Mouse(MouseEvent),
}

impl TryFrom<event::Event> for Event {
    type Error = InputError;

    fn try_from(event: event::Event) -> result::Result<Self, Self::Error> {
        match event {
            event::Event::Key(ke) => Ok(Event::Key(ke.try_into()?)),
            event::Event::Mouse(me) => Ok(Event::Mouse(me.try_into()?)),
            _ => Err(InputError::UnsupportedEvent),
        }
    }
}

#[derive(Default)]
pub struct KeyEvent {
    pub char: char,
    pub modifier: Option<KeyModifier>,
}

pub enum KeyModifier {
    Ctrl,
}

impl TryFrom<event::KeyEvent> for KeyEvent {
    type Error = InputError;

    fn try_from(event: event::KeyEvent) -> result::Result<Self, Self::Error> {
        let event::KeyEvent { code, modifiers } = event;
        match code {
            event::KeyCode::Char(char) => Ok(Self {
                char,
                modifier: match modifiers {
                    event::KeyModifiers::CONTROL => Some(KeyModifier::Ctrl),
                    _ => None,
                },
            }),
            _ => Err(InputError::UnsupportedKeyEvent),
        }
    }
}

pub struct MouseEvent {
    pub(crate) action: MouseAction,
    pub(crate) pos: grid::Point,
}

impl MouseEvent {
    pub fn new(action: MouseAction, pos: grid::Point) -> Self {
        Self { action, pos }
    }
}

pub enum MouseAction {
    Press,
    Drag,
    Release,
}

impl TryFrom<event::MouseEvent> for MouseEvent {
    type Error = InputError;

    fn try_from(event: event::MouseEvent) -> result::Result<Self, Self::Error> {
        let mouse = |x, y, action| Ok(MouseEvent::new(action, grid::Point::new(x, y)));
        match event {
            event::MouseEvent::Down(_, x, y, _) => mouse(x, y, MouseAction::Press),
            event::MouseEvent::Up(_, x, y, _) => mouse(x, y, MouseAction::Release),
            event::MouseEvent::Drag(_, x, y, _) => mouse(x, y, MouseAction::Drag),
            _ => Err(InputError::UnsupportedMouseEvent),
        }
    }
}
