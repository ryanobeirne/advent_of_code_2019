pub mod intcode;
pub use intcode::*;

use std::error::Error;
use std::fmt;

pub type BoxError = Box<dyn Error>;
pub type Result<T> = std::result::Result<T, BoxError>;

#[macro_export]
macro_rules! answer {
    ($day: expr, $part: expr, $answer: expr) => {
        println!("Day {}, Part {}: {}", $day, stringify!($part), $answer);
    }
}

#[macro_export]
macro_rules! boxerr {
    ($err: expr) => {
        Err(Box::new($err))
    }
}

#[macro_export]
macro_rules! ioerr {
    ($error: expr) => {
        boxerr!(InputError::new($error))
    }
}

#[derive(Debug)]
pub struct InputError<T: fmt::Debug> {
    input: T,
}

impl<T: fmt::Debug> InputError<T> {
    pub fn new(input: T) -> Self {
        InputError { input }
    }
}

impl<T: fmt::Debug> fmt::Display for InputError<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid input: '{:?}'", self.input)
    }
}

impl<T: fmt::Debug> Error for InputError<T> {}
