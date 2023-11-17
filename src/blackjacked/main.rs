use std::error::Error;

use clap::{arg, Args};

use crate::blackjacked::cards::{Card, Deck};
use crate::blackjacked::error::BJError;
use crate::blackjacked::game::{Action, Game, Outcome};

/// A thug's pocket BlackJack program
#[derive(Args)]
pub struct Options {
    /// Specify number of decks
    #[arg(short, long, value_name = "NUM", default_value_t = 1)]
    decks: u8,
}

pub fn main(opts: &Options) -> Result<(), Box<dyn Error>> {
    let mut game = Game::new(opts.decks);
    draw_hands(&mut game).unwrap();
    loop {
        let play = play_cards(&mut game);

        match play {
            Outcome::DealerBust => {
                println!("Dealer busted, you WIN!");
                draw_hands(&mut game).unwrap_or_else(|_| {
                    println!("Deck finished, recreating...");
                    game.deck.recreate();
                });
                game.dealer_done = false;
                continue;
            }
            Outcome::DealerWin => {
                println!("Dealer got BlackJack! You LOSE!");
                draw_hands(&mut game).unwrap_or_else(|_| {
                    println!("Deck finished, recreating...");
                    game.deck.recreate();
                });
                game.dealer_done = false;
                continue;
            }
            Outcome::PlayerBust => {
                println!("You busted, you LOSE!");
                draw_hands(&mut game).unwrap_or_else(|_| {
                    println!("Deck finished, recreating...");
                    game.deck.recreate();
                });
                game.dealer_done = false;
                continue;
            }
            Outcome::PlayerWin => {
                println!("You got BlackJack! You WIN!");
                draw_hands(&mut game).unwrap_or_else(|_| {
                    println!("Deck finished, recreating...");
                    game.deck.recreate();
                });
                game.dealer_done = false;
                continue;
            }
            Outcome::Draw => {
                println!("Draw. No one wins");
                draw_hands(&mut game).unwrap_or_else(|_| {
                    println!("Deck finished, recreating...");
                    game.deck.recreate();
                });
                game.dealer_done = false;
                continue;
            }
            Outcome::None => {}
        }
        let action = ask_input();
        match action {
            Ok(a) => match a {
                Action::Hit => {
                    if let Ok(card) = draw_card(&mut game.deck) {
                        game.player_hand.add(card);
                    } else {
                        println!("Deck finished, recreating...");
                        game.deck.recreate();
                        game.dealer_done = false;
                        continue;
                    }
                }
                Action::Stand => {}
            },
            Err(e) => {
                eprintln!("Error: {}", e);
                // Restart loop and ask for input again
                continue;
            }
        }
        while game.dealer_hand.value() < 17 {
            println!("Dealer hand draw BEGIN ({})", game.dealer_hand.value());
            if let Ok(card) = draw_card(&mut game.deck) {
                game.dealer_hand.add(card);
            } else {
                println!("Deck finished, recreating...");
                game.deck.recreate();
                game.dealer_done = false;
                continue;
            }
            println!("Dealer hand draw DONE ({})", game.dealer_hand.value());
        }
        // Mark dealer as done with moves
        game.dealer_done = true;
    }
}

/// Display Cards and Check for busts and blackjacks
fn play_cards(game: &mut Game) -> Outcome {
    // Print Dealer and Player hand status
    print!("Dealer Hand {}: ", game.dealer_hand.value());
    println!(
        "{}",
        game.dealer_hand
            .cards
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<String>>()
            .join(" ")
    );
    print!("Player Hand {}: ", game.player_hand.value());
    println!(
        "{}",
        game.player_hand
            .cards
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<String>>()
            .join(" ")
    );

    // Check for BlackJacks and busts
    if game.dealer_hand.value() == 21 {
        Outcome::DealerWin
    } else if game.player_hand.value() == 21 {
        Outcome::PlayerWin
    } else if game.player_hand.value() > 21 {
        Outcome::PlayerBust
    } else if game.dealer_hand.value() > 21 {
        Outcome::DealerBust
    } else if game.dealer_done && game.dealer_hand.value() == game.player_hand.value() {
        Outcome::Draw
    } else {
        Outcome::None
    }
}

/// Ask for input from std
fn ask_input() -> Result<Action, BJError> {
    println!("Choose Action: (H)it (S)tand");

    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .map_err(|_| BJError::UserInput)?;

    let action = input.trim().to_uppercase();
    match action.as_str() {
        "H" => Ok(Action::Hit),
        "S" => Ok(Action::Stand),
        _ => Err(BJError::ActionUnknown),
    }
}

/// Draw hands with new cards
fn draw_hands(game: &mut Game) -> Result<(), BJError> {
    game.dealer_hand.clear();
    game.player_hand.clear();
    for _ in 0..2 {
        let dealer_card = game.deck.draw()?;
        let player_hand = game.deck.draw()?;
        game.dealer_hand.add(dealer_card);
        game.player_hand.add(player_hand);
    }
    Ok(())
}

/// Draw card from Deck
fn draw_card(deck: &mut Deck) -> Result<Card, BJError> {
    let card = deck.draw()?;
    Ok(card)
}
