use std::error;
use std::result;

pub(crate) type Error = Box<dyn error::Error>;
pub(crate) type Result<T = ()> = result::Result<T, Error>;

pub mod grid;

mod canvas;
mod terminal;

pub use canvas::*;
pub use terminal::*;
