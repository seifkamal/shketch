use std::fmt;
use std::io::Write;
use termion::cursor;
use termion::event::{Event, MouseEvent};

#[derive(Clone, Copy)]
#[repr(u8)]
enum Char {
    Stationary = b'.',
    Vertical = b'|',
    Horizontal = b'_',
    DiagonalRight = b'\\',
    DiagonalLeft = b'/',
}

impl fmt::Display for Char {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.clone() as u8 as char)
    }
}

struct Cursor {
    x: u16,
    y: u16,
    char: Char,
}

impl Cursor {
    fn new(x: u16, y: u16) -> Self {
        Self { x, y, char: Char::Stationary }
    }

    fn update(&mut self, x: u16, y: u16) {
        self.char = match (x, y) {
            (x, y) if x == self.x && y != self.y => Char::Vertical,
            (x, y) if x != self.x && y == self.y => Char::Horizontal,
            (x, y) if (x > self.x && y > self.y) || (x < self.x && y < self.y) => Char::DiagonalRight,
            (x, y) if (x > self.x && y < self.y) || (x < self.x && y > self.y) => Char::DiagonalLeft,
            _ => Char::Stationary,
        };

        self.x = x;
        self.y = y;
    }

    fn plot<W: Write>(&mut self, writer: &mut W) {
        // Reserve first line for usage instructions
        if self.y == 1 {
            return;
        }

        write!(writer, "{}{}", cursor::Goto(self.x, self.y), self.char).unwrap();
    }
}

pub trait Brush {
    fn paint<W: Write>(&mut self, writer: &mut W, event: Event);
}

pub struct FreeBrush {
    cursor: Cursor,
}

impl FreeBrush {
    pub fn new() -> Self {
        Self { cursor: Cursor::new(1, 2) }
    }
}

impl Brush for FreeBrush {
    fn paint<W: Write>(&mut self, writer: &mut W, event: Event) {
        match event {
            Event::Mouse(mouse_event) => match mouse_event {
                MouseEvent::Press(_, a, b) |
                MouseEvent::Release(a, b) |
                MouseEvent::Hold(a, b) => {
                    self.cursor.update(a, b);
                    self.cursor.plot(writer);
                    writer.flush().unwrap();
                }
            }
            _ => {}
        }
    }
}

pub struct LineBrush {
    cursor: Cursor,
}

impl LineBrush {
    pub fn new() -> Self {
        Self { cursor: Cursor::new(1, 2) }
    }
}

impl Brush for LineBrush {
    fn paint<W: Write>(&mut self, writer: &mut W, event: Event) {
        match event {
            Event::Mouse(mouse_event) => match mouse_event {
                MouseEvent::Press(_, a, b) => self.cursor.update(a, b),
                MouseEvent::Release(x, y) => {
                    let x_diff = (x as i16) - (self.cursor.x as i16);
                    if x_diff < 0 {
                        // negative, move left
                        for _ in x_diff..0 {
                            self.cursor.update(self.cursor.x - 1, self.cursor.y);
                            self.cursor.plot(writer);
                        }
                    } else {
                        // positive, move right
                        for _ in 0..x_diff {
                            self.cursor.update(self.cursor.x + 1, self.cursor.y);
                            self.cursor.plot(writer);
                        }
                    }

                    let y_diff = (y as i16) - (self.cursor.y as i16);
                    if y_diff < 0 {
                        // negative, move up
                        for _ in y_diff..0 {
                            self.cursor.update(self.cursor.x, self.cursor.y - 1);
                            self.cursor.plot(writer);
                        }
                    } else {
                        // positive, move down
                        for _ in 0..y_diff {
                            self.cursor.update(self.cursor.x, self.cursor.y + 1);
                            self.cursor.plot(writer);
                        }
                    }

                    writer.flush().unwrap();
                }
                _ => {}
            }
            _ => {}
        }
    }
}
