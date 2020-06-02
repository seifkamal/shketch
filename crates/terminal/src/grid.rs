use std::cmp;
use std::fmt;
use std::io::{self, Write};
use std::iter;
use std::ops;

use crate::style;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Point {
    pub x: u16,
    pub y: u16,
}

impl Point {
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }

    pub fn move_up(&mut self) {
        self.y -= 1;
    }

    pub fn move_down(&mut self) {
        self.y += 1;
    }

    pub fn move_left(&mut self) {
        self.x -= 1;
    }

    pub fn move_right(&mut self) {
        self.x += 1;
    }

    pub fn move_to(&mut self, x: u16, y: u16) {
        self.x = x;
        self.y = y;
    }
}

impl Default for Point {
    fn default() -> Self {
        Self { x: 1, y: 1 }
    }
}

impl From<(u16, u16)> for Point {
    fn from(tuple: (u16, u16)) -> Self {
        Self::new(tuple.0, tuple.1)
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\x1B[{};{}H", self.y, self.x)
    }
}

pub trait Erase {
    fn erase(&mut self, writer: &mut impl Write) -> io::Result<()>;
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
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.pos, self.content)
    }
}

impl Erase for Cell {
    fn erase(&mut self, writer: &mut impl Write) -> io::Result<()> {
        self.content = ' ';
        write!(writer, "{}", self)
    }
}

#[derive(Debug, Default, Clone)]
pub struct Segment {
    cells: Vec<Cell>,
    format: style::Format,
}

impl Segment {
    pub fn new() -> Self {
        Self { cells: Vec::new(), format: Default::default() }
    }

    pub fn from_str(start: Point, str: &str, format: style::Format) -> Self {
        let mut cells = Vec::new();
        let mut cursor = start;
        for char in str.as_bytes() {
            cells.push(Cell::new(cursor, (*char) as char));
            cursor.move_right();
        }

        Self { cells, format }
    }

    pub fn add(&mut self, cell: Cell) {
        self.cells.push(cell);
    }

    pub fn clear(&mut self) {
        self.cells.clear();
    }

    pub fn set_format(&mut self, format: style::Format) {
        self.format = format;
    }

    pub fn boundaries(&self) -> Option<(Point, Point)> {
        if self.cells.is_empty() {
            return None;
        }

        let x_s = self.cells.iter().map(|cell| cell.pos.x);
        let y_s = self.cells.iter().map(|cell| cell.pos.y);

        Some((
            Point::new(
                x_s.clone().min().expect("could not determine min segment x"),
                y_s.clone().min().expect("could not determine min segment y"),
            ),
            Point::new(
                x_s.max().expect("could not determine max segment x"),
                y_s.max().expect("could not determine max segment y"),
            ),
        ))
    }
}

impl From<Segment> for String {
    fn from(segment: Segment) -> Self {
        let mut output = "".to_string();

        let boundaries = segment.boundaries();
        if boundaries.is_none() {
            return output;
        }

        let (start, end) = boundaries.unwrap();
        let mut cursor = start;

        while cursor.y <= end.y {
            cursor.move_to(start.x, cursor.y);
            while cursor.x <= end.x {
                match segment.cells.iter().find(|cell| cell.pos == cursor) {
                    Some(cell) => output.push(cell.content),
                    None => output.push(' '),
                }
                cursor.move_right();
            }
            output.push('\n');
            cursor.move_down();
        }

        output
    }
}

impl<'a> iter::Sum<&'a Segment> for Segment {
    fn sum<I: Iterator<Item = &'a Segment>>(iter: I) -> Self {
        let mut result = Segment::new();
        for segment in iter {
            result += segment.clone()
        }

        result
    }
}

impl ops::AddAssign for Segment {
    fn add_assign(&mut self, mut rhs: Self) {
        self.cells.append(rhs.cells.as_mut())
    }
}

impl fmt::Display for Segment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format)?;
        for cell in &self.cells {
            write!(f, "{}", cell)?;
        }
        write!(f, "{}", style::RESET_FORMAT)?;
        Ok(())
    }
}

impl Erase for Segment {
    fn erase(&mut self, writer: &mut impl Write) -> io::Result<()> {
        for cell in &mut self.cells {
            cell.erase(writer)?;
        }
        Ok(())
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
    pub fn next(&self, from: Point, to: Point) -> char {
        let Point { x, y } = to;
        let Point { x: cx, y: cy } = from;

        match (x, y) {
            (x, y) if cx == x && cy < y => self.up,
            (x, y) if cx == x && cy > y => self.down,
            (x, y) if cx < x && cy == y => self.left,
            (x, y) if cx > x && cy == y => self.right,
            (x, y) if (cx > x && cy > y) || (cx < x && cy < y) => self.diagonal_back,
            (x, y) if (cx > x && cy < y) || (cx < x && cy > y) => self.diagonal_forward,
            _ => self.stationary,
        }
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

#[derive(Debug, Default)]
pub struct Tracer {
    char_set: CharSet,
}

impl Tracer {
    pub fn new(char_set: CharSet) -> Self {
        Self { char_set }
    }

    pub fn trace(&self, from: Point, to: Point) -> Segment {
        let mut segment = Segment::new();
        let mut cursor = from;

        while cursor != to {
            let current_pos = cursor;

            match cursor.y.cmp(&to.y) {
                cmp::Ordering::Greater => cursor.move_up(),
                cmp::Ordering::Less => cursor.move_down(),
                _ => {}
            };

            match cursor.x.cmp(&to.x) {
                cmp::Ordering::Greater => cursor.move_left(),
                cmp::Ordering::Less => cursor.move_right(),
                _ => {}
            };

            segment.add(Cell::new(cursor, self.char_set.next(current_pos, cursor)));
        }

        segment
    }
}
