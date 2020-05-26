use std::convert::{TryFrom, TryInto};
use std::error;
use std::fmt;
use std::io;
use std::result;

use crossterm::{event, ErrorKind};
use crossterm::ExecutableCommand;
use crossterm::terminal;
use crossterm::tty::IsTty;

type SomeResult<T = ()> = result::Result<T, Box<dyn error::Error>>;
type ExecResult<'a> = SomeResult<&'a mut Terminal>;

pub fn is_tty() -> bool {
    io::stdout().is_tty() && io::stdin().is_tty()
}

pub struct Terminal {
    stdout: io::Stdout,
}

impl Terminal {
    pub fn new(stdout: io::Stdout) -> Self {
        Self { stdout }
    }

    pub fn read_event(&self) -> Result<Event, InputError> {
        event::read()?.try_into()
    }

    pub fn enable_raw_mode(&mut self) -> ExecResult {
        terminal::enable_raw_mode()?;
        Ok(self)
    }

    pub fn disable_raw_mode(&mut self) -> ExecResult {
        terminal::disable_raw_mode()?;
        Ok(self)
    }

    pub fn hide_cursor(&mut self) -> ExecResult {
        self.stdout.execute(crossterm::cursor::Hide)?;
        Ok(self)
    }

    pub fn show_cursor(&mut self) -> ExecResult {
        self.stdout.execute(crossterm::cursor::Show)?;
        Ok(self)
    }

    pub fn enter_alt_screen(&mut self) -> ExecResult {
        self.stdout.execute(terminal::EnterAlternateScreen)?;
        Ok(self)
    }

    pub fn leave_alt_screen(&mut self) -> ExecResult {
        self.stdout.execute(terminal::LeaveAlternateScreen)?;
        Ok(self)
    }

    pub fn enable_mouse_capture(&mut self) -> ExecResult {
        self.stdout.execute(event::EnableMouseCapture)?;
        Ok(self)
    }

    pub fn disable_mouse_capture(&mut self) -> ExecResult {
        self.stdout.execute(event::DisableMouseCapture)?;
        Ok(self)
    }

    pub fn clear(&mut self) -> ExecResult {
        self.stdout.execute(terminal::Clear(terminal::ClearType::All))?;
        Ok(self)
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
    UnknownError(crossterm::ErrorKind),
}

impl error::Error for InputError {}

impl From<crossterm::ErrorKind> for InputError {
    fn from(ct_error: ErrorKind) -> Self {
        InputError::UnknownError(ct_error)
    }
}

impl fmt::Display for InputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InputError::UnsupportedKeyEvent |
            InputError::UnsupportedMouseEvent |
            InputError::UnsupportedEvent => write!(f, "unsupported input event"),
            InputError::UnknownError(e) => write!(f, "some error occurred; {}", e),
        }
    }
}

pub enum Event {
    Key(KeyEvent),
    Mouse(MouseEvent),
}

impl TryFrom<event::Event> for Event {
    type Error = InputError;

    fn try_from(event: event::Event) -> Result<Self, Self::Error> {
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

    fn try_from(event: event::KeyEvent) -> Result<Self, Self::Error> {
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
    pub pos: MousePos,
    pub action: MouseAction,
}

pub type MousePos = (u16, u16);

pub enum MouseAction {
    Press,
    Drag,
    Release,
}

impl MouseEvent {
    pub fn new(pos: MousePos, action: MouseAction) -> Self {
        Self { action, pos }
    }
}

impl TryFrom<event::MouseEvent> for MouseEvent {
    type Error = InputError;

    fn try_from(event: event::MouseEvent) -> Result<Self, Self::Error> {
        let mouse = |x, y, action| Ok(MouseEvent::new((x, y), action));
        match event {
            event::MouseEvent::Down(_, x, y, _) => mouse(x, y, MouseAction::Press),
            event::MouseEvent::Up(_, x, y, _) => mouse(x, y, MouseAction::Release),
            event::MouseEvent::Drag(_, x, y, _) => mouse(x, y, MouseAction::Drag),
            _ => Err(InputError::UnsupportedMouseEvent),
        }
    }
}
