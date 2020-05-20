use std::cmp;

use crate::component;

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
    pub fn next(&self, from: component::Point, to: component::Point) -> char {
        let component::Point { x, y } = to;
        let component::Point { x: cx, y: cy } = from;

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
    fn connect(&self, from: component::Point, to: component::Point) -> component::Segment;
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
    fn connect(&self, from: component::Point, to: component::Point) -> component::Segment {
        let mut segment = component::Segment::new();
        let mut cursor = from;

        while cursor != to {
            let current_pos = cursor;

            match cursor.y().cmp(&to.y()) {
                cmp::Ordering::Greater => cursor.move_up(),
                cmp::Ordering::Less => cursor.move_down(),
                _ => {}
            };

            match cursor.x().cmp(&to.x()) {
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
