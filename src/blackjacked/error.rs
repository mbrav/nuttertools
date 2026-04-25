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
            Self::DeckEmpty => write!(f, "Deck is empty"),
            Self::ActionUnknown => write!(f, "Action is not known"),
            Self::UserInput => write!(f, "Failed to get user input"),
        }
    }
}
