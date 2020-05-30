use std::io;

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
    let mut save_file_name: Option<String> = None;
    let mut canvas = grid::Canvas::new(io::stdout(), grid::Tracer::default());
    let toolbar = {
        let format = terminal::Format::new(terminal::Color::White, terminal::Color::Black);
        let item = |x, y, text| grid::Segment::from_str((x, y).into(), text, format);
        // Actions
        let mut toolbar = item(1, 1, "Exit (q)");
        toolbar += item(15, 1, "Clear (k)");
        toolbar += item(30, 1, "Undo (u)");
        toolbar += item(45, 1, "Save (Ctrl+s)");
        // Tools
        toolbar += item(1, 2, "Plot (1)");
        toolbar += item(15, 2, "Line (2)");
        toolbar
    };

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
                                ('1', _) => canvas.alt_style(grid::Style::Plot),
                                ('2', _) => canvas.alt_style(grid::Style::Line),
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                }

                canvas.draw()?;
                canvas.overlay(&toolbar)?;
            }
            Err(terminal::InputError::UnknownError(error)) => return Err(error.into()),
            Err(terminal::InputError::UnsupportedEvent) => {}
        }
    }

    Ok(())
}
