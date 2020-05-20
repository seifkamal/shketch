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

    pub fn boundaries(&self) -> Option<(Point, Point)> {
        if self.cells.is_empty() {
            return None;
        }

        Some((
            Point::new(
                self.cells
                    .iter()
                    .map(|cell| cell.pos.x)
                    .min()
                    .expect("could not determine min segment x"),
                self.cells
                    .iter()
                    .map(|cell| cell.pos.y)
                    .min()
                    .expect("could not determine min segment y"),
            ),
            Point::new(
                self.cells
                    .iter()
                    .map(|cell| cell.pos.x)
                    .max()
                    .expect("could not determine max segment x"),
                self.cells
                    .iter()
                    .map(|cell| cell.pos.y)
                    .max()
                    .expect("could not determine max segment y"),
            ),
        ))
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
