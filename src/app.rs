use std::io::{self, Write};

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
    let mut canvas = grid::Canvas::new(io::stdout(), grid::Tracer::default());
    let mut save_file_name: Option<String> = None;
    let mut toolbar = menu::ToolBar::new();
    let mut screen = io::stdout();

    loop {
        match terminal.read_event() {
            Ok(event) => {
                if let Some(event) = event {
                    match event {
                        // Reserve toolbar space
                        terminal::Event::Mouse(event) if event.pos.1 > 3 => canvas.update(event)?,
                        terminal::Event::Key(terminal::KeyEvent { char, modifier }) => {
                            match (char, modifier) {
                                ('q', _) => break,
                                ('u', _) => canvas.undo()?,
                                ('k', _) => canvas.clear()?,
                                ('s', Some(terminal::KeyModifier::Ctrl)) => {
                                    let blueprint: grid::Segment = canvas.snapshot().iter().sum();
                                    match save_file_name {
                                        Some(ref name) => export::to_file_as(&blueprint, name)?,
                                        None => save_file_name = Some(export::to_file(&blueprint)?),
                                    }
                                }
                                (n, _) if n.is_digit(10) => {
                                    let style = match n {
                                        '2' => grid::Style::Line,
                                        _ => grid::Style::Plot,
                                    };

                                    canvas.alt_style(style);
                                    toolbar.highlight_tool(style);
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                }

                write!(screen, "{}", canvas)?;
                write!(screen, "{}", toolbar)?;
                screen.flush()?;
            }
            Err(terminal::InputError::UnknownError(error)) => return Err(error.into()),
            Err(terminal::InputError::UnsupportedEvent) => {}
        }
    }

    Ok(())
}

mod menu {
    use std::collections::HashMap;
    use std::fmt;

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
        tools: HashMap<grid::Style, grid::Segment>,
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

            let mut tools: HashMap<grid::Style, grid::Segment> = HashMap::new();
            tools.insert(grid::Style::Plot, str_to_segment((1, 2), "Plot (1)"));
            tools.insert(grid::Style::Line, str_to_segment((15, 2), "Line (2)"));

            let mut bar = Self { actions, tools };
            bar.highlight_tool(Default::default());
            bar
        }

        pub fn highlight_tool(&mut self, tool: grid::Style) {
            for (style, segment) in &mut self.tools {
                match *style == tool {
                    true => segment.set_format(HIGHLIGHT_FORMAT),
                    false => segment.set_format(Default::default()),
                }
            }
        }
    }

    impl fmt::Display for ToolBar {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.actions)?;
            for (_, segment) in &self.tools {
                write!(f, "{}", segment)?;
            }
            Ok(())
        }
    }
}
