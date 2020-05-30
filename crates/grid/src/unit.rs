use std::fmt;
use std::io::{self, Write};
use std::iter;
use std::ops;

use crate::path;

pub trait Erase {
    fn erase(&mut self, writer: &mut impl Write) -> io::Result<()>;
}

#[derive(Debug, Default, Copy, Clone)]
pub struct Cell {
    pos: path::Point,
    content: char,
}

impl Cell {
    pub fn new(pos: path::Point, content: char) -> Self {
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
    format: terminal::Format,
}

impl Segment {
    pub fn new() -> Self {
        Self { cells: Vec::new(), format: Default::default() }
    }

    pub fn from_str(start: path::Point, str: &str, format: terminal::Format) -> Self {
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

    pub fn boundaries(&self) -> Option<(path::Point, path::Point)> {
        if self.cells.is_empty() {
            return None;
        }

        let x_s = self.cells.iter().map(|cell| cell.pos.x);
        let y_s = self.cells.iter().map(|cell| cell.pos.y);

        Some((
            path::Point::new(
                x_s.clone().min().expect("could not determine min segment x"),
                y_s.clone().min().expect("could not determine min segment y"),
            ),
            path::Point::new(
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
        write!(f, "{}", terminal::RESET_COLOR)?;
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
