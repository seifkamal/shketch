use std::fmt;
use std::io::{self, Write};

use terminal::grid::{self, Erase};

use crate::export;

pub fn launch() -> crate::Result {
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

    let result = run_canvas(&mut terminal);

    terminal
        .clear()?
        .show_cursor()?
        .disable_mouse_capture()?
        .disable_raw_mode()?
        .leave_alt_screen()?;

    result
}

fn run_canvas(terminal: &mut terminal::Terminal) -> crate::Result {
    let mut screen = io::stdout();
    let mut canvas = Canvas::new();
    let mut toolbar = menu::ToolBar::new();
    let mut save_file_name: Option<String> = None;

    let mut sketch = grid::Segment::new();
    let mut style = Style::default();
    let brush = grid::Tracer::default();

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
                                        segment.erase(&mut screen)?;
                                    }
                                }
                                ('k', _) => {
                                    canvas.clear();
                                    sketch.clear();
                                    terminal.clear()?;
                                }
                                ('s', Some(terminal::KeyModifier::Ctrl)) => {
                                    let blueprint: grid::Segment = canvas.snapshot().iter().sum();
                                    match save_file_name {
                                        Some(ref name) => export::to_file_as(&blueprint, name)?,
                                        None => save_file_name = Some(export::to_file(&blueprint)?),
                                    }
                                }
                                (n, _) if n.is_digit(10) => {
                                    style = match n {
                                        '2' => Style::Line,
                                        _ => Style::Plot,
                                    };

                                    toolbar.highlight_tool(style);
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
                                (terminal::MouseAction::Drag, (x, y)) => match style {
                                    Style::Plot => {
                                        sketch += brush.connect(canvas.cursor, (x, y).into());
                                        canvas.cursor.move_to(x, y);
                                    }
                                    Style::Line => {
                                        sketch.erase(&mut screen)?;
                                        sketch = brush.connect(canvas.cursor, (x, y).into());
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

                write!(screen, "{}{}{}", canvas, sketch, toolbar)?;
                screen.flush()?;
            }
            Err(terminal::InputError::UnknownError(error)) => return Err(error.into()),
            Err(terminal::InputError::UnsupportedEvent) => {}
        }
    }

    Ok(())
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Style {
    Plot,
    Line,
}

impl Default for Style {
    fn default() -> Self {
        Style::Plot
    }
}

#[derive(Debug, Default)]
struct Canvas {
    cursor: grid::Point,
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

mod menu {
    use std::collections::HashMap;
    use std::fmt;

    use terminal::grid;

    #[rustfmt::skip]
    static HIGHLIGHT_FORMAT: terminal::Format = terminal::Format {
        bg_color: terminal::Color::White,
        fg_color: terminal::Color::Black,
    };

    fn str_to_segment((x, y): (u16, u16), text: &str) -> grid::Segment {
        grid::Segment::from_str(grid::Point::new(x, y), text, Default::default())
    }

    pub(super) struct ToolBar {
        actions: grid::Segment,
        tools: HashMap<super::Style, grid::Segment>,
    }

    impl ToolBar {
        pub fn new() -> Self {
            let actions: grid::Segment = vec![
                str_to_segment((1, 1), "Exit (q)"),
                str_to_segment((15, 1), "Clear (k)"),
                str_to_segment((30, 1), "Undo (u)"),
                str_to_segment((45, 1), "Save (Ctrl+s)"),
            ]
            .iter()
            .sum();

            let mut tools: HashMap<super::Style, grid::Segment> = HashMap::new();
            tools.insert(super::Style::Plot, str_to_segment((1, 2), "Plot (1)"));
            tools.insert(super::Style::Line, str_to_segment((15, 2), "Line (2)"));

            let mut toolbar = Self { actions, tools };
            toolbar.highlight_tool(Default::default());
            toolbar
        }

        pub fn highlight_tool(&mut self, tool: super::Style) {
            for (style, segment) in &mut self.tools {
                if *style == tool {
                    segment.set_format(HIGHLIGHT_FORMAT);
                } else {
                    segment.set_format(Default::default());
                }
            }
        }
    }

    impl fmt::Display for ToolBar {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.actions)?;
            for segment in self.tools.values() {
                write!(f, "{}", segment)?;
            }
            Ok(())
        }
    }
}
