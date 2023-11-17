use std::fmt;

use crate::blackjacked::error::BJError;
use crate::blackjacked::util::shuffle_array;

/// Struct representation of house cards
pub struct Deck {
    pub decks: u8,
    pub cards: Vec<Card>,
}

/// Deck implementation
impl Deck {
    /// Create new Deck
    pub fn new(decks: u8) -> Deck {
        let mut house_deck = Deck {
            decks,
            cards: generate_cards(decks),
        };
        house_deck.shuffle();
        house_deck
    }

    /// Recreate deck with newly generated cards
    pub fn recreate(&mut self) {
        self.cards = generate_cards(self.decks);
    }

    /// Shuffle cards
    pub fn shuffle(&mut self) {
        shuffle_array(&mut self.cards)
    }

    /// Draw from deck
    pub fn draw(&mut self) -> Result<Card, BJError> {
        self.cards.pop().ok_or(BJError::DeckEmpty)
    }
}

/// Get an Array of all possible Cards
fn generate_cards(decks: u8) -> Vec<Card> {
    let mut cards: Vec<Card> = vec![];

    for _ in 0..decks {
        let deck: Vec<Card> = VALUES
            .iter()
            .flat_map(|v| {
                SUITS.iter().map(|s| Card {
                    suit: s.clone(),
                    value: v.clone(),
                })
            })
            .collect();
        cards.extend(deck);
    }

    cards
}

/// Card suits
#[derive(Debug, Clone)]
pub enum Suit {
    Diamonds,
    Clubs,
    Hearts,
    Spades,
}

/// Card values
#[derive(Debug, Clone)]
pub enum Value {
    Ace,
    /// For cards from 2 to 10
    Number(u8),
    Jack,
    Queen,
    King,
}

/// All possible card Suits
static SUITS: &[Suit] = &[Suit::Diamonds, Suit::Clubs, Suit::Hearts, Suit::Spades];

/// All possible card values
static VALUES: &[Value] = &[
    Value::Ace,
    Value::Number(2),
    Value::Number(3),
    Value::Number(4),
    Value::Number(5),
    Value::Number(6),
    Value::Number(7),
    Value::Number(8),
    Value::Number(9),
    Value::Number(10),
    Value::Jack,
    Value::Queen,
    Value::King,
];

/// Card data struct
#[derive(Debug)]
pub struct Card {
    pub value: Value,
    pub suit: Suit,
}

/// Card implementation
impl Card {
    /// Get Card numeric value
    pub fn val(&self) -> u8 {
        match self.value {
            Value::Ace => 11,
            Value::Number(num) => num,
            Value::Jack => 10,
            Value::Queen => 10,
            Value::King => 10,
        }
    }
}

/// Card data struct implementation for Display
impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (&self.value, &self.suit) {
            (Value::Ace, Suit::Diamonds) => write!(f, "🃁"),
            (Value::Ace, Suit::Clubs) => write!(f, "🃑"),
            (Value::Ace, Suit::Hearts) => write!(f, "🂱"),
            (Value::Ace, Suit::Spades) => write!(f, "🂡"),
            (Value::Number(2), Suit::Diamonds) => write!(f, "🃂"),
            (Value::Number(2), Suit::Clubs) => write!(f, "🃒"),
            (Value::Number(2), Suit::Hearts) => write!(f, "🂲"),
            (Value::Number(2), Suit::Spades) => write!(f, "🂢"),
            (Value::Number(3), Suit::Diamonds) => write!(f, "🃃"),
            (Value::Number(3), Suit::Clubs) => write!(f, "🃓"),
            (Value::Number(3), Suit::Hearts) => write!(f, "🂳"),
            (Value::Number(3), Suit::Spades) => write!(f, "🂣"),
            (Value::Number(4), Suit::Diamonds) => write!(f, "🃄"),
            (Value::Number(4), Suit::Clubs) => write!(f, "🃔"),
            (Value::Number(4), Suit::Hearts) => write!(f, "🂴"),
            (Value::Number(4), Suit::Spades) => write!(f, "🂤"),
            (Value::Number(5), Suit::Diamonds) => write!(f, "🃅"),
            (Value::Number(5), Suit::Clubs) => write!(f, "🃕"),
            (Value::Number(5), Suit::Hearts) => write!(f, "🂵"),
            (Value::Number(5), Suit::Spades) => write!(f, "🂥"),
            (Value::Number(6), Suit::Diamonds) => write!(f, "🃆"),
            (Value::Number(6), Suit::Clubs) => write!(f, "🃖"),
            (Value::Number(6), Suit::Hearts) => write!(f, "🂶"),
            (Value::Number(6), Suit::Spades) => write!(f, "🂦"),
            (Value::Number(7), Suit::Diamonds) => write!(f, "🃇"),
            (Value::Number(7), Suit::Clubs) => write!(f, "🃗"),
            (Value::Number(7), Suit::Hearts) => write!(f, "🂷"),
            (Value::Number(7), Suit::Spades) => write!(f, "🂧"),
            (Value::Number(8), Suit::Diamonds) => write!(f, "🃈"),
            (Value::Number(8), Suit::Clubs) => write!(f, "🃘"),
            (Value::Number(8), Suit::Hearts) => write!(f, "🂸"),
            (Value::Number(8), Suit::Spades) => write!(f, "🂨"),
            (Value::Number(9), Suit::Diamonds) => write!(f, "🃉"),
            (Value::Number(9), Suit::Clubs) => write!(f, "🃙"),
            (Value::Number(9), Suit::Hearts) => write!(f, "🂹"),
            (Value::Number(9), Suit::Spades) => write!(f, "🂩"),
            (Value::Number(10), Suit::Diamonds) => write!(f, "🃊"),
            (Value::Number(10), Suit::Clubs) => write!(f, "🃚"),
            (Value::Number(10), Suit::Hearts) => write!(f, "🂺"),
            (Value::Number(10), Suit::Spades) => write!(f, "🂪"),
            (Value::Jack, Suit::Diamonds) => write!(f, "🃋"),
            (Value::Jack, Suit::Clubs) => write!(f, "🃛"),
            (Value::Jack, Suit::Hearts) => write!(f, "🂻"),
            (Value::Jack, Suit::Spades) => write!(f, "🂫"),
            (Value::Queen, Suit::Diamonds) => write!(f, "🃍"),
            (Value::Queen, Suit::Clubs) => write!(f, "🃝"),
            (Value::Queen, Suit::Hearts) => write!(f, "🂽"),
            (Value::Queen, Suit::Spades) => write!(f, "🂭"),
            (Value::King, Suit::Diamonds) => write!(f, "🃎"),
            (Value::King, Suit::Clubs) => write!(f, "🃞"),
            (Value::King, Suit::Hearts) => write!(f, "🂾"),
            (Value::King, Suit::Spades) => write!(f, "🂮"),
            (Value::Number(n), _) => panic!(
                "Display not implemented for card. Number {} is likely not a valid numeric value for card", n
            ),
        }
    }
}
