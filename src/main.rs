use std::io::stdin;
use termion::input::TermRead;
use termion::event::{Key, Event, MouseEvent};

use ward::{self, grid::{self, Connect}};

enum Tool {
    Brush,
    Ruler,
}

fn main() {
    let mut frame = ward::Frame::default();
    let mut cursor = grid::Point::default();
    let mut segment = grid::Segment::new();
    let tracer = grid::Tracer::default();

    let mut tool = Tool::Brush;

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
                        match tool {
                            Tool::Brush => {
                                segment += tracer.connect(&cursor, &grid::Point::new(a, b));
                                cursor = grid::Point::new(a, b);
                            }
                            Tool::Ruler => {
                                frame.erase(segment);
                                segment = tracer.connect(&cursor, &grid::Point::new(a, b));
                            }
                        }
                    }
                    MouseEvent::Release(_, _) => {
                        frame.add(segment.clone());
                        segment.clear();
                    }
                }
            }
            _ => {}
        }

        frame.print();
        frame.layer(&segment);
    }
}
