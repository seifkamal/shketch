use std::fmt;
use std::io::Write;

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

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        (self.x == other.x) && (self.y == other.y)
    }
}

#[derive(Debug, Copy, Clone)]
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
        write!(f, "\x1B[{};{}H{}", self.pos.y, self.pos.x, self.content)
    }
}

pub fn clear_cell<W: Write>(mut cell: Cell, writer: &mut W) {
    cell.content = ' ';
    write!(writer, "{}", cell).unwrap();
}

#[derive(Debug, Clone)]
pub struct Segment {
    cells: Vec<Cell>
}

impl Segment {
    pub fn new() -> Self {
        Self { cells: Vec::new() }
    }

    pub fn add(&mut self, cell: Cell) {
        self.cells.push(cell);
    }

    pub fn clear(&mut self) {
        self.cells.clear();
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
    fn connect(&self, segment: &Segment, point: &Point) -> Result<Segment, &str>;
}

pub struct Tracer {
    char_set: CharSet
}

impl Connect for Tracer {
    fn connect(&self, segment: &Segment, point: &Point) -> Result<Segment, &str> {
        let cell = segment.cells.last();
        if cell.is_none() {
            return Err("Grid segment must contain at least one cell for a connection to be made");
        }

        let mut cursor = cell.unwrap().pos().clone();
        let mut new_segment = segment.clone();

        while cursor != *point {
            let current_pos = cursor.clone();

            if cursor.y > point.y {
                cursor = cursor.up();
            } else if cursor.y < point.y {
                cursor = cursor.down();
            }

            if cursor.x > point.x {
                cursor = cursor.left();
            } else if cursor.x < point.x {
                cursor = cursor.right();
            }

            new_segment.add(Cell::new(cursor, self.char_set.next(&current_pos, &cursor)));
        }

        Ok(new_segment)
    }
}

impl Default for Tracer {
    fn default() -> Self {
        Self { char_set: CharSet::default() }
    }
}
