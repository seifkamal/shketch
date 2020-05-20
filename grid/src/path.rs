use std::cmp;
use std::fmt;

use crate::component;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Point {
    pub(crate) x: u16,
    pub(crate) y: u16,
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

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\x1B[{};{}H", self.y, self.x)
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

pub trait Connect {
    fn connect(&self, from: Point, to: Point) -> component::Segment;
}

pub struct Tracer {
    char_set: CharSet,
}

impl Default for Tracer {
    fn default() -> Self {
        Self {
            char_set: CharSet::default(),
        }
    }
}

impl Connect for Tracer {
    fn connect(&self, from: Point, to: Point) -> component::Segment {
        let mut segment = component::Segment::new();
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

            segment.add(component::Cell::new(
                cursor,
                self.char_set.next(current_pos, cursor),
            ));
        }

        segment
    }
}
