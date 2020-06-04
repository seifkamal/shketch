use std::fmt;

use terminal::grid;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Tool {
    Plot,
    Line,
    Erase,
}

impl Default for Tool {
    fn default() -> Self {
        Tool::Plot
    }
}

#[derive(Debug, Default)]
pub struct Canvas {
    pub cursor: grid::Point,
    design: Vec<grid::Segment>,
}

impl Canvas {
    pub fn new() -> Self {
        Self { design: Vec::new(), cursor: Default::default() }
    }

    pub fn add(&mut self, segment: grid::Segment) {
        self.design.push(segment)
    }

    pub fn undo(&mut self) -> Option<grid::Segment> {
        self.design.pop()
    }

    pub fn clear(&mut self) {
        self.design.iter_mut().for_each(|segment| segment.clear());
    }

    pub fn snapshot(&self) -> Vec<grid::Segment> {
        self.design.clone()
    }
}

impl fmt::Display for Canvas {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.design.iter().try_for_each(|segment| write!(f, "{}", segment))
    }
}
