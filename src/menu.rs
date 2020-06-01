use std::collections::HashMap;
use std::fmt;

use terminal::grid;

use crate::canvas;

#[rustfmt::skip]
static HIGHLIGHT_FORMAT: terminal::Format = terminal::Format {
    bg_color: terminal::Color::White,
    fg_color: terminal::Color::Black,
};

pub struct ToolBar {
    actions: grid::Segment,
    tools: HashMap<canvas::Tool, grid::Segment>,
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

        let mut tools: HashMap<canvas::Tool, grid::Segment> = HashMap::new();
        tools.insert(canvas::Tool::Plot, str_to_segment((1, 2), "Plot (1)"));
        tools.insert(canvas::Tool::Line, str_to_segment((15, 2), "Line (2)"));

        let mut toolbar = Self { actions, tools };
        toolbar.highlight_tool(Default::default());
        toolbar
    }

    pub fn highlight_tool(&mut self, tool: canvas::Tool) {
        for (menu_tool, segment) in &mut self.tools {
            if *menu_tool == tool {
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

fn str_to_segment((x, y): (u16, u16), text: &str) -> grid::Segment {
    grid::Segment::from_str(grid::Point::new(x, y), text, Default::default())
}
