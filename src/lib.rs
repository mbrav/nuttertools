use std::fmt;

pub mod phone_gen;
pub mod prosecho;
pub mod proxy_police;
pub mod rat;
pub mod russian_roulette;

/// nuttertools Error
#[derive(Debug)]
pub enum Error {
    ConnectionError,
    SomeError,
    ValueError,
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ConnectionError => write!(f, "Connection Error"),
            Self::SomeError => write!(f, "Some Error"),
            Self::ValueError => write!(f, "Value Error"),
        }
    }
}
