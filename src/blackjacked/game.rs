use crate::blackjacked::cards::{Card, Deck, Value};

use std::collections::VecDeque;

/// Struct representation of Game State
pub struct Game {
    pub deck: Deck,
    pub dealer_hand: Hand,
    pub dealer_done: bool,
    pub player_hand: Hand,
}

/// Game action
#[derive(Debug)]
pub enum Action {
    Hit,
    Stand,
}

/// Game outcome
#[derive(Debug)]
pub enum Outcome {
    DealerBust,
    DealerWin,
    PlayerBust,
    PlayerWin,
    Draw,
    None,
}

/// Game implementation
/// TODO: Move game logic to main.rs
impl Game {
    /// Create new Game
    pub fn new(decks: u8) -> Game {
        let deck = Deck::new(decks);
        deck.cards.iter().for_each(|c| {
            print!("{} ", c);
        });
        println!(" ");
        Game {
            deck,
            player_hand: Hand::new(),
            dealer_done: false,
            dealer_hand: Hand::new(),
        }
    }

    // /// Draw card from deck to a provided hand
    // fn deal_card(&mut self, hand: &mut Hand) -> Result<(), BJError> {
    //     let card = self.deck.draw()?;
    //     hand.add(card);
    //     Ok(())
    // }
}

/// Struct representation of Hand
pub struct Hand {
    pub cards: VecDeque<Card>,
}

/// Hand implementation
impl Hand {
    /// Create new Game
    pub fn new() -> Hand {
        Hand {
            cards: VecDeque::new(),
        }
    }

    /// Add Card to Hand
    pub fn add(&mut self, card: Card) {
        self.cards.push_back(card);
    }

    /// Get the sum of values in hand
    pub fn value(&self) -> u8 {
        // Count number of aces in hand
        let mut aces = self
            .cards
            .iter()
            // Don't count Aces since 11 => 1 conversion checks are necessary
            .filter(|c| matches!(c.value, Value::Ace))
            .count();

        // Get sum of all cards
        let mut val = self.cards.iter().map(|v| v.val()).sum();

        // If convert aces from 11 to 1 if value is over 21
        while val > 21 && aces > 0 {
            val -= 10;
            aces -= 1;
        }

        val
    }

    pub fn clear(&mut self) {
        self.cards.clear();
    }
}
