use std::error;
use std::fmt;
use std::fs;
use std::io::{self, Write};
use std::path;
use std::time;

type SaveResult<T> = Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    SystemTime(time::SystemTimeError),
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::Io(e) => Some(e),
            Error::SystemTime(e) => Some(e),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self {
            Error::Io(_) => "failed to perform I/O operation",
            Error::SystemTime(_) => "could not get current system time",
        };

        write!(f, "could not save design; {}", msg)
    }
}

impl From<io::Error> for Error {
    fn from(io_error: io::Error) -> Self {
        Error::Io(io_error)
    }
}

impl From<time::SystemTimeError> for Error {
    fn from(sys_time_error: time::SystemTimeError) -> Self {
        Error::SystemTime(sys_time_error)
    }
}

pub fn to_file(blueprint: &grid::Segment) -> SaveResult<String> {
    let file_name = {
        let time = time::SystemTime::now().duration_since(time::SystemTime::UNIX_EPOCH)?;
        format!("shketch-{}.txt", time.as_millis())
    };
    to_file_as(blueprint, &file_name)?;
    Ok(file_name)
}

pub fn to_file_as(blueprint: &grid::Segment, file_name: &str) -> SaveResult<()> {
    let mut file = fs::File::create(path::Path::new(&file_name))?;
    let content: String = blueprint.clone().into();
    file.write_all(content.as_bytes())?;
    Ok(())
}
