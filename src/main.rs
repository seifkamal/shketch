use std::convert::TryInto;
use std::io::{stdin, stdout};

use termion::event::{Event, Key};
use termion::input::{MouseTerminal, TermRead};
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;

use shketch::{export, grid, Canvas};

fn main() {
    let design;

    {
        let screen = AlternateScreen::from(stdout().into_raw_mode().unwrap());
        let mut canvas = Canvas::new(MouseTerminal::from(screen), grid::Tracer::default());

        canvas.pin(toolbar());
        canvas.draw();

        for c in stdin().events() {
            match c.unwrap() {
                Event::Key(Key::Char('q')) => break,
                Event::Key(Key::Char('k')) => canvas.clear(),
                Event::Key(Key::Char('u')) => canvas.undo(),
                Event::Key(Key::Char(n)) if n.is_digit(10) => canvas.alt_style(n.into()),
                Event::Mouse(mouse_event) => canvas.update(mouse_event),
                _ => {}
            }

            canvas.draw();
        }

        design = canvas.snapshot();
    }

    let blueprint: Result<export::BluePrint, _> = design.try_into();
    println!("{}", blueprint.expect("Could not export design"));
}

fn toolbar() -> grid::Segment {
    let item = |x, y, text| grid::Segment::from_str(grid::Point::new(x, y), text);

    let mut toolbar = grid::Segment::new();
    toolbar += item(1, 1, "q - Exit");
    toolbar += item(20, 1, "k - Clear");
    toolbar += item(40, 1, "u - Undo");
    toolbar += item(1, 2, "1 - Brush");
    toolbar += item(20, 2, "2 - Ruler");
    toolbar
}
