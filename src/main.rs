use std::io::{stdin, stdout};

use termion::event::{Key, Event, MouseEvent};
use termion::input::{MouseTerminal, TermRead};
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;

use shketch::{self, Connect};

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
    let screen = AlternateScreen::from(stdout().into_raw_mode().unwrap());
    let mut frame = shketch::Frame::new(MouseTerminal::from(screen));

    {
        let mut toolbar = shketch::Segment::new();
        toolbar += shketch::Segment::from_str(shketch::Point::new(1, 1), "q - Exit");
        toolbar += shketch::Segment::from_str(shketch::Point::new(20, 1), "k - Clear");
        toolbar += shketch::Segment::from_str(shketch::Point::new(40, 1), "u - Undo");
        toolbar += shketch::Segment::from_str(shketch::Point::new(1, 2), "1 - Brush");
        toolbar += shketch::Segment::from_str(shketch::Point::new(20, 2), "2 - Ruler");
        frame.layer(&toolbar);

        let tracer = shketch::Tracer::default();
        let mut sketch = shketch::Segment::new();
        let mut cursor = shketch::Point::default();
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
                            cursor = shketch::Point::new(a, b);
                        }
                        MouseEvent::Hold(a, b) => {
                            // Reserve toolbar space
                            if b < 3 {
                                continue;
                            }

                            match tool {
                                Tool::Brush => {
                                    sketch += tracer.connect(&cursor, &shketch::Point::new(a, b));
                                    cursor = shketch::Point::new(a, b);
                                }
                                Tool::Ruler => {
                                    frame.erase(sketch);
                                    sketch = tracer.connect(&cursor, &shketch::Point::new(a, b));
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
}
