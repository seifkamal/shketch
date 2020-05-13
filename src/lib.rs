pub mod grid;

use std::io::{stdout, Stdout, Write};
use termion::{
    clear,
    cursor,
    input::MouseTerminal,
    screen::AlternateScreen,
    raw::{RawTerminal, IntoRawMode},
};

#[derive(Debug, Clone)]
pub struct Frame<W: Write> {
    writer: W,
    segments: Vec<grid::Segment>,
}

impl<W: Write> Frame<W> {
    pub fn new(mut writer: W) -> Self {
        write!(&mut writer, "{}{}", termion::clear::All, termion::cursor::Hide).unwrap();
        writer.flush().unwrap();

        Self { writer, segments: Vec::new() }
    }

    pub fn print(&mut self) {
        for segment in &self.segments {
            write!(self.writer, "{}", segment).unwrap();
        }
        self.writer.flush().unwrap();
    }

    pub fn layer(&mut self, segment: &grid::Segment) {
        write!(self.writer, "{}", segment).unwrap();
        self.writer.flush().unwrap();
    }

    pub fn add(&mut self, segment: grid::Segment) {
        self.segments.push(segment);
    }

    pub fn undo(&mut self) {
        if let Some(segment) = self.segments.pop() {
            grid::clear_segment(segment, &mut self.writer);
        }
    }

    pub fn erase(&mut self, segment: grid::Segment) {
        grid::clear_segment(segment, &mut self.writer);
    }

    pub fn clear(&mut self) {
        self.segments.clear();
        write!(self.writer, "{}", clear::All).unwrap();
        self.writer.flush().unwrap();
    }
}

impl<W: Write> Drop for Frame<W> {
    fn drop(&mut self) {
        write!(self.writer, "{}{}", clear::All, cursor::Show).unwrap();
    }
}

type AltMouseTerm = MouseTerminal<AlternateScreen<RawTerminal<Stdout>>>;

impl Default for Frame<AltMouseTerm> {
    fn default() -> Self {
        let screen = AlternateScreen::from(stdout().into_raw_mode().unwrap());
        Self::new(MouseTerminal::from(screen))
    }
}
