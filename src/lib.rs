use std::error::Error;

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
    () => {
        Err(Box::new(std::io::Error::from(std::io::ErrorKind::InvalidInput)))
    }
}

