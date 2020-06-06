use std::io::{self, Write};

use terminal::grid::{self, Erase};

use crate::canvas;
use crate::export;
use crate::menu;

pub struct Opts {
    char_set: grid::CharSet,
}

impl Opts {
    pub fn new(char_set: grid::CharSet) -> Self {
        Self { char_set }
    }
}

pub fn launch(opts: Opts) -> crate::Result {
    if !terminal::is_tty() {
        return Err("stream is not TTY".into());
    }

    let mut terminal = terminal::Terminal::default();

    terminal
        .enter_alt_screen()?
        .enable_raw_mode()?
        .enable_mouse_capture()?
        .hide_cursor()?
        .clear()?;

    let result = run_canvas(&mut terminal, opts.char_set);

    terminal
        .clear()?
        .show_cursor()?
        .disable_mouse_capture()?
        .disable_raw_mode()?
        .leave_alt_screen()?;

    result
}

fn run_canvas(terminal: &mut terminal::Terminal, char_set: grid::CharSet) -> crate::Result {
    let mut screen = io::stdout();
    let mut canvas = canvas::Canvas::new();
    let mut sketch = grid::Segment::new();
    let mut toolbar = menu::ToolBar::new();
    let mut tool = canvas::Tool::default();
    let mut file_name: Option<String> = None;
    let mut file_name_print = grid::Segment::new();

    let tracer = grid::Tracer::new(char_set);

    loop {
        match terminal.read_event() {
            Ok(event) => {
                if let Some(event) = event {
                    match event {
                        terminal::Event::Key(terminal::KeyEvent { char, modifier }) => {
                            match (char, modifier) {
                                ('q', _) => break,
                                ('u', _) => {
                                    if let Some(mut segment) = canvas.undo() {
                                        screen.erase(&mut segment)?;
                                    }
                                }
                                ('k', _) => {
                                    canvas.clear();
                                    sketch.clear();
                                    terminal.clear()?;
                                }
                                ('s', Some(terminal::KeyModifier::Ctrl)) => {
                                    let blueprint: grid::Segment = canvas.snapshot().iter().sum();
                                    match file_name {
                                        Some(ref name) => export::to_file_as(blueprint, name)?,
                                        None => {
                                            let name = export::to_file(blueprint)?;
                                            file_name_print = grid::Segment::from_str(
                                                (1, 300).into(),
                                                &name,
                                                terminal::Format::new(
                                                    terminal::Color::Black,
                                                    terminal::Color::Green,
                                                ),
                                            );

                                            file_name = Some(name);
                                        }
                                    }
                                }
                                (n, _) if n.is_digit(10) => {
                                    tool = match n {
                                        '3' => canvas::Tool::Erase,
                                        '2' => canvas::Tool::Line,
                                        _ => canvas::Tool::Plot,
                                    };

                                    toolbar.highlight_tool(tool);
                                }
                                _ => {}
                            }
                        }
                        // Reserve toolbar space
                        terminal::Event::Mouse(event) if event.pos.1 > 3 => {
                            match (event.action, event.pos) {
                                (terminal::MouseAction::Press, (x, y)) => {
                                    canvas.cursor.move_to(x, y)
                                }
                                (terminal::MouseAction::Drag, (x, y)) => match tool {
                                    canvas::Tool::Plot => {
                                        sketch += tracer.trace(canvas.cursor, (x, y).into());
                                        canvas.cursor.move_to(x, y);
                                    }
                                    canvas::Tool::Line => {
                                        screen.erase(&mut sketch)?;
                                        sketch = tracer.trace(canvas.cursor, (x, y).into());
                                    }
                                    canvas::Tool::Erase => {
                                        sketch.add(grid::Cell::new((x, y).into(), ' '));
                                    }
                                },
                                (terminal::MouseAction::Release, _) => {
                                    canvas.add(sketch.clone());
                                    sketch.clear();
                                }
                            }
                        }
                        _ => {}
                    }
                }

                write!(screen, "{}{}{}{}", canvas, sketch, toolbar, file_name_print)?;
                screen.flush()?;
            }
            Err(terminal::InputError::UnknownError(error)) => return Err(error.into()),
            Err(terminal::InputError::UnsupportedEvent) => {}
        }
    }

    Ok(())
}
