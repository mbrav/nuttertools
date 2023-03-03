use std::fmt;

pub mod phone_gen;
pub mod prosecho;
pub mod russian_roulette;
pub mod rat;

/// nuttertools Error
#[derive(Debug)]
pub enum Error {
    SomeError,
    ValueError,
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::SomeError => write!(f, "Some Error"),
            Self::ValueError => write!(f, "Value Error"),
        }
    }
}
