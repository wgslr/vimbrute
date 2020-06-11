use std::error;
use std::fmt;
use std::fs;
use std::io;
use vimdecrypt;

pub mod cli;

#[derive(Debug, Clone)]
pub enum Error {
    ArgsError,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::ArgsError => write!(f, "Error parsing arguments"),
        }
    }
}

impl error::Error for Error {}
