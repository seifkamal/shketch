use std::io::stdin;
use termion::input::TermRead;
use termion::event::{Key, Event, MouseEvent};

use ward::{self, grid::{self, Connect}};

enum Tool {
    Brush,
    Ruler,
}

impl Default for Tool {
    fn default() -> Self {
        Tool::Brush
    }
}

fn main() {
    let mut frame = ward::Frame::default();
    let mut sketch = grid::Segment::new();
    let mut toolbar = grid::Segment::new();
    toolbar += grid::Segment::from_str(grid::Point::new(1, 1), "q - Exit");
    toolbar += grid::Segment::from_str(grid::Point::new(20, 1), "k - Clear");
    toolbar += grid::Segment::from_str(grid::Point::new(40, 1), "u - Undo");
    toolbar += grid::Segment::from_str(grid::Point::new(1, 2), "1 - Brush");
    toolbar += grid::Segment::from_str(grid::Point::new(20, 2), "2 - Ruler");
    frame.layer(&toolbar);

    let tracer = grid::Tracer::default();
    let mut cursor = grid::Point::default();
    let mut tool = Tool::default();

    for c in stdin().events() {
        match c.unwrap() {
            Event::Key(Key::Char('q')) => break,
            Event::Key(Key::Char('k')) => frame.clear(),
            Event::Key(Key::Char('u')) => frame.undo(),
            Event::Key(Key::Char('1')) => tool = Tool::Brush,
            Event::Key(Key::Char('2')) => tool = Tool::Ruler,
            Event::Mouse(mouse_event) => {
                match mouse_event {
                    MouseEvent::Press(_, a, b) => {
                        cursor = grid::Point::new(a, b);
                    }
                    MouseEvent::Hold(a, b) => {
                        // Reserve toolbar space
                        if b < 3 {
                            continue;
                        }

                        match tool {
                            Tool::Brush => {
                                sketch += tracer.connect(&cursor, &grid::Point::new(a, b));
                                cursor = grid::Point::new(a, b);
                            }
                            Tool::Ruler => {
                                frame.erase(sketch);
                                sketch = tracer.connect(&cursor, &grid::Point::new(a, b));
                            }
                        }
                    }
                    MouseEvent::Release(_, _) => {
                        frame.add(sketch.clone());
                        sketch.clear();
                    }
                }
            }
            _ => {}
        }

        frame.print();
        frame.layer(&sketch);
        frame.layer(&toolbar);
    }
}
