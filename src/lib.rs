use std::error;
use std::result;

pub(crate) type Error = Box<dyn error::Error>;
pub(crate) type Result<T = ()> = result::Result<T, Error>;

pub mod app;
pub(crate) mod canvas;
pub(crate) mod export;
pub(crate) mod menu;
