use std::io;

use crate::export;

pub fn run() -> crate::Result {
    if !terminal::is_tty() {
        return Err("stream is not TTY".into());
    }

    let mut terminal = terminal::Terminal::default();
    init_terminal(&mut terminal)?;

    {
        let mut save_file_name: Option<String> = None;
        let mut canvas = grid::Canvas::new(io::stdout(), grid::Tracer::default());
        canvas.pin(toolbar());
        canvas.draw()?;

        loop {
            match terminal.read_event().unwrap() {
                terminal::Event::Mouse(me) => canvas.update(me)?,
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
                        (n, _) if n.is_digit(10) => canvas.alt_style(n.into()),
                        _ => {}
                    }
                }
            }

            canvas.draw()?;
        }
    }

    restore_terminal(&mut terminal)
}

fn init_terminal(terminal: &mut terminal::Terminal) -> crate::Result {
    terminal
        .enter_alt_screen()?
        .enable_raw_mode()?
        .enable_mouse_capture()?
        .hide_cursor()?
        .clear()?;
    Ok(())
}

fn restore_terminal(terminal: &mut terminal::Terminal) -> crate::Result {
    terminal
        .clear()?
        .show_cursor()?
        .disable_mouse_capture()?
        .disable_raw_mode()?
        .leave_alt_screen()?;
    Ok(())
}

fn toolbar() -> grid::Segment {
    let item = |x, y, text| grid::Segment::from_str((x, y).into(), text);
    let mut toolbar = grid::Segment::new();
    // Actions
    toolbar += item(1, 1, "Exit (q)");
    toolbar += item(15, 1, "Clear (k)");
    toolbar += item(30, 1, "Undo (u)");
    toolbar += item(45, 1, "Save (Ctrl+s)");
    // Tools
    toolbar += item(1, 2, "Brush (1)");
    toolbar += item(15, 2, "Ruler (2)");
    toolbar
}
