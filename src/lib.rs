use std::fmt;
use std::io::Write;

use termion::{clear, cursor};

#[derive(Debug, Copy, Clone, Eq, Hash)]
pub struct Point {
    x: u16,
    y: u16,
}

impl Point {
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }

    pub fn up(&self) -> Self {
        Self { x: self.x, y: self.y - 1 }
    }

    pub fn down(&self) -> Self {
        Self { x: self.x, y: self.y + 1 }
    }

    pub fn left(&self) -> Self {
        Self { x: self.x - 1, y: self.y }
    }

    pub fn right(&self) -> Self {
        Self { x: self.x + 1, y: self.y }
    }
}

impl Default for Point {
    fn default() -> Self {
        Self { x: 1, y: 1 }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        (self.x == other.x) && (self.y == other.y)
    }
}

#[derive(Debug, Default, Copy, Clone)]
pub struct Cell {
    pos: Point,
    content: char,
}

impl Cell {
    pub fn new(pos: Point, content: char) -> Self {
        Self { pos, content }
    }

    pub fn pos(&self) -> &Point {
        &self.pos
    }

    pub fn content(&self) -> &char {
        &self.content
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", cursor::Goto(self.pos.x, self.pos.y), self.content)
    }
}

pub fn clear_cell<W: Write>(mut cell: Cell, writer: &mut W) {
    cell.content = ' ';
    write!(writer, "{}", cell).unwrap();
}

#[derive(Debug, Default, Clone)]
pub struct Segment {
    cells: Vec<Cell>
}

impl Segment {
    pub fn new() -> Self {
        Self { cells: Vec::new() }
    }

    pub fn from_str(start: Point, str: &str) -> Self {
        let mut cells = Vec::new();
        let mut cursor = start;
        for char in str.as_bytes() {
            cells.push(Cell::new(cursor.clone(), (*char) as char));
            cursor = cursor.right();
        }

        Self { cells }
    }

    pub fn add(&mut self, cell: Cell) {
        self.cells.push(cell);
    }

    pub fn clear(&mut self) {
        self.cells.clear();
    }
}

impl From<Vec<Cell>> for Segment {
    fn from(cells: Vec<Cell>) -> Self {
        Self { cells }
    }
}

impl std::ops::AddAssign for Segment {
    fn add_assign(&mut self, mut rhs: Self) {
        self.cells.append(rhs.cells.as_mut())
    }
}

impl fmt::Display for Segment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for cell in &self.cells {
            write!(f, "{}", cell)?;
        }
        Ok(())
    }
}

pub fn clear_segment<W: Write>(segment: Segment, writer: &mut W) {
    for cell in segment.cells {
        clear_cell(cell, writer);
    }
}

#[derive(Debug)]
pub struct CharSet {
    stationary: char,
    up: char,
    down: char,
    left: char,
    right: char,
    diagonal_back: char,
    diagonal_forward: char,
}

impl CharSet {
    pub fn next(&self, from: &Point, to: &Point) -> char {
        return match *to {
            Point { x, y } if from.x == x && from.y < y => self.up,
            Point { x, y } if from.x == x && from.y > y => self.down,
            Point { x, y } if from.x < x && from.y == y => self.left,
            Point { x, y } if from.x > x && from.y == y => self.right,
            Point { x, y } if (from.x > x && from.y > y) || (from.x < x && from.y < y) => self.diagonal_back,
            Point { x, y } if (from.x > x && from.y < y) || (from.x < x && from.y > y) => self.diagonal_forward,
            _ => self.stationary,
        };
    }
}

impl Default for CharSet {
    fn default() -> Self {
        Self {
            stationary: '.',
            up: '|',
            down: '|',
            left: '_',
            right: '_',
            diagonal_back: '\\',
            diagonal_forward: '/',
        }
    }
}

pub trait Connect {
    fn connect(&self, from: &Point, to: &Point) -> Segment;
}

pub struct Tracer {
    char_set: CharSet
}

impl Connect for Tracer {
    fn connect(&self, from: &Point, to: &Point) -> Segment {
        let mut segment = Segment::new();
        let mut cursor = from.clone();

        while cursor != *to {
            let current_pos = cursor.clone();

            if cursor.y > to.y {
                cursor = cursor.up();
            } else if cursor.y < to.y {
                cursor = cursor.down();
            }

            if cursor.x > to.x {
                cursor = cursor.left();
            } else if cursor.x < to.x {
                cursor = cursor.right();
            }

            segment.add(Cell::new(cursor, self.char_set.next(&current_pos, &cursor)));
        }

        segment
    }
}

impl Default for Tracer {
    fn default() -> Self {
        Self { char_set: CharSet::default() }
    }
}

#[derive(Debug, Clone)]
pub struct Frame<W: Write> {
    writer: W,
    segments: Vec<Segment>,
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

    pub fn layer(&mut self, segment: &Segment) {
        write!(self.writer, "{}", segment).unwrap();
        self.writer.flush().unwrap();
    }

    pub fn add(&mut self, segment: Segment) {
        self.segments.push(segment);
    }

    pub fn undo(&mut self) {
        if let Some(segment) = self.segments.pop() {
            clear_segment(segment, &mut self.writer);
        }
    }

    pub fn erase(&mut self, segment: Segment) {
        clear_segment(segment, &mut self.writer);
    }

    pub fn clear(&mut self) {
        self.segments.clear();
        write!(self.writer, "{}", clear::All).unwrap();
        self.writer.flush().unwrap();
    }
}

impl<W: Write> Drop for Frame<W> {
    fn drop(&mut self) {
        write!(self.writer, "{}{}{}", clear::All, cursor::Goto(1, 1), cursor::Show).unwrap();
    }
}
