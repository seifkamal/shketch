use std::io::stdin;
use termion::input::TermRead;
use termion::event::{Key, Event, MouseEvent};

use ward::{self, grid::{self, Connect}};

fn main() {
    let mut frame = ward::Frame::default();

    {
        let tracer = grid::Tracer::default();
        let mut segment = grid::Segment::new();

        for c in stdin().events() {
            match c.unwrap() {
                Event::Key(Key::Char('q')) => break,
                Event::Key(Key::Char('k')) => frame.clear(),
                Event::Key(Key::Char('u')) => frame.undo(),
                Event::Mouse(mouse_event) => {
                    match mouse_event {
                        MouseEvent::Press(_, a, b) => {
                            segment.add(grid::Cell::new(grid::Point::new(a, b), ' '));
                        }
                        MouseEvent::Hold(a, b) => {
                            segment = tracer.connect(&segment, &grid::Point::new(a, b)).unwrap();
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
}
