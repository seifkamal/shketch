use std::convert;
use std::fmt;
use std::fs;
use std::io::Write;
use std::path;
use std::time;

use crate::grid;

#[derive(Debug)]
pub struct BluePrint {
    boundaries: (grid::Point, grid::Point),
    cells: Vec<grid::Cell>,
}

impl BluePrint {
    fn new(boundaries: (grid::Point, grid::Point), cells: Vec<grid::Cell>) -> Self {
        Self { boundaries, cells }
    }
}

impl convert::TryFrom<Vec<grid::Segment>> for BluePrint {
    type Error = &'static str;

    fn try_from(design: Vec<grid::Segment>) -> Result<Self, Self::Error> {
        if design.is_empty() {
            return Err("Design is empty");
        }

        let unified_segment: grid::Segment = design.iter().sum();
        let cells: Vec<grid::Cell> = unified_segment.into();
        let boundaries = (
            grid::Point::new(
                cells.iter().map(|cell| cell.pos().x()).min().unwrap(),
                cells.iter().map(|cell| cell.pos().y()).min().unwrap(),
            ),
            grid::Point::new(
                cells.iter().map(|cell| cell.pos().x()).max().unwrap(),
                cells.iter().map(|cell| cell.pos().y()).max().unwrap(),
            ),
        );

        Ok(BluePrint::new(boundaries, cells))
    }
}

impl From<&BluePrint> for String {
    fn from(frame: &BluePrint) -> Self {
        let (start, end) = frame.boundaries;
        let mut cursor = start;
        let mut output = String::from("");

        while cursor.y() <= end.y() {
            cursor.move_to(start.x(), cursor.y());
            while cursor.x() <= end.x() {
                match frame.cells.iter().find(|cell| *cell.pos() == cursor) {
                    Some(cell) => output.push(cell.content()),
                    None => output.push(' '),
                };

                cursor.move_right();
            }
            output.push('\n');
            cursor.move_down();
        }

        output
    }
}

impl fmt::Display for BluePrint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let output: String = self.into();
        write!(f, "{}", output)
    }
}

type SaveResult<T> = Result<T, Box<dyn std::error::Error>>;

pub fn save(blueprint: &BluePrint) -> SaveResult<String> {
    let time = time::SystemTime::now().duration_since(time::SystemTime::UNIX_EPOCH)?;
    let file_name = format!("shketch-{}", time.as_millis());
    save_as(blueprint, &file_name)?;
    Ok(file_name)
}

pub fn save_as(blueprint: &BluePrint, file_name: &str) -> SaveResult<()> {
    let path = path::Path::new(&file_name);
    let mut file = fs::File::create(path)?;
    file.write_all(blueprint.to_string().as_bytes())?;
    Ok(())
}
