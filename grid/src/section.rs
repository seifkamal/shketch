use std::fmt;
use std::io::{self, Write};
use std::iter;
use std::ops;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Point {
    pub(crate) x: u16,
    pub(crate) y: u16,
}

impl Point {
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }

    pub fn x(self) -> u16 {
        self.x
    }

    pub fn y(self) -> u16 {
        self.y
    }

    pub fn move_to(&mut self, x: u16, y: u16) {
        self.x = x;
        self.y = y;
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
}

impl Default for Point {
    fn default() -> Self {
        Self { x: 1, y: 1 }
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

    pub fn pos(&self) -> &Point {
        &self.pos
    }

    pub fn content(self) -> char {
        self.content
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\x1B[{};{}H{}", self.pos.y, self.pos.x, self.content)
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
}

impl Segment {
    pub fn new() -> Self {
        Self { cells: Vec::new() }
    }

    pub fn from_str(start: Point, str: &str) -> Self {
        let mut cells = Vec::new();
        let mut cursor = start;
        for char in str.as_bytes() {
            cells.push(Cell::new(cursor, (*char) as char));
            cursor.move_right();
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

impl From<Segment> for Vec<Cell> {
    fn from(segment: Segment) -> Self {
        segment.cells
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
        for cell in &self.cells {
            write!(f, "{}", cell)?;
        }
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
