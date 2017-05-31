extern crate rusqlite;
extern crate tempfile;
#[macro_use]
extern crate clap;

pub mod history;
pub mod cli;
mod error;

pub use error::Error;

pub type Result<T> = std::result::Result<T, Error>;
