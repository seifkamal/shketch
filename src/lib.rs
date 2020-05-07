pub mod paint;

use std::io::{Stdout, stdout, stdin, Write};
use termion::{
    clear,
    cursor,
    event::{Event, Key},
    input::{MouseTerminal, TermRead},
    raw::{IntoRawMode, RawTerminal},
    screen::{AlternateScreen},
};

pub struct Canvas<W: Write> {
    writer: W
}

impl<W: Write> Canvas<W> {
    pub fn new(writer: W) -> Self {
        Self { writer }
    }

    pub fn start<B: paint::Brush>(&mut self, mut brush: B) {
        for c in stdin().events() {
            match c.unwrap() {
                Event::Key(Key::Char('q')) => break,
                Event::Key(Key::Char('k')) => self.clear(),
                event => brush.paint(&mut self.writer, event)
            }
        }
    }

    pub fn clear(&mut self) {
        write!(
            self.writer,
            "{}{}q to exit. k to clear.{}{}",
            clear::All,
            cursor::Goto(1, 1),
            cursor::Goto(1, 2),
            cursor::Hide
        ).unwrap();
        self.writer.flush().unwrap();
    }
}

impl<W: Write> Drop for Canvas<W> {
    fn drop(&mut self) {
        writeln!(
            self.writer,
            "{}{}{}",
            clear::All,
            cursor::Goto(1, 1),
            cursor::Show
        ).unwrap();
    }
}

type AltMouseTerm = MouseTerminal<AlternateScreen<RawTerminal<Stdout>>>;

impl Default for Canvas<AltMouseTerm> {
    fn default() -> Self {
        let screen = AlternateScreen::from(stdout().into_raw_mode().unwrap());
        Self::new(MouseTerminal::from(screen))
    }
}
