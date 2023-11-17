use std::fmt;

/// Custom defined Error
#[derive(Debug)]
pub enum BJError {
    DeckEmpty,
    ActionUnknown,
    UserInput,
}

impl fmt::Display for BJError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BJError::DeckEmpty => write!(f, "Deck is empty"),
            BJError::ActionUnknown => write!(f, "Action is not known"),
            BJError::UserInput => write!(f, "Failed to get user input"),
        }
    }
}
