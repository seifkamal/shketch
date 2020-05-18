use std::convert;
use std::error;
use std::fmt;
use std::fs;
use std::io::{self, Write};
use std::path;
use std::time;

use crate::grid;

#[derive(Debug)]
pub enum ParseError {
    EmptyDesign,
}

impl error::Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self {
            ParseError::EmptyDesign => "Design is empty",
        };

        write!(f, "{}", msg)
    }
}

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
    type Error = ParseError;

    fn try_from(design: Vec<grid::Segment>) -> Result<Self, Self::Error> {
        if design.is_empty() {
            return Err(ParseError::EmptyDesign);
        }

        let unified_segment: grid::Segment = design.iter().sum();
        let cells: Vec<grid::Cell> = unified_segment.into();
        let boundaries = (
            grid::Point::new(
                cells
                    .iter()
                    .map(|cell| cell.pos().x())
                    .min()
                    .expect("Find min x"),
                cells
                    .iter()
                    .map(|cell| cell.pos().y())
                    .min()
                    .expect("Find min y"),
            ),
            grid::Point::new(
                cells
                    .iter()
                    .map(|cell| cell.pos().x())
                    .max()
                    .expect("Find max x"),
                cells
                    .iter()
                    .map(|cell| cell.pos().y())
                    .max()
                    .expect("Find max y"),
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

type SaveResult<T> = Result<T, SaveError>;

#[derive(Debug)]
pub enum SaveError {
    Io(io::Error),
    SystemTime(time::SystemTimeError),
}

impl error::Error for SaveError {}

impl fmt::Display for SaveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self {
            SaveError::Io(_) => "failed to perform I/O operation",
            SaveError::SystemTime(_) => "could not get current system time",
        };

        write!(f, "Could not save design: {}", msg)
    }
}

impl From<io::Error> for SaveError {
    fn from(io_error: io::Error) -> Self {
        SaveError::Io(io_error)
    }
}

impl From<time::SystemTimeError> for SaveError {
    fn from(sys_time_error: time::SystemTimeError) -> Self {
        SaveError::SystemTime(sys_time_error)
    }
}

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
