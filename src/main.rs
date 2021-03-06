mod reversi;

use reversi::*;
use std::io::{Write,stdout};

fn main() {
    println!("Choose players.");
    println!("1 => human (from keyboard)");
    println!("2 => AI (alpha-beta search)");
    println!("3 => random");

    print!("First player? Type either key in [123] (human by default): ");
    stdout().flush().unwrap();
    let black = match read_one_char() {
        Some('1') => Box::new(HumanPlayer::new()) as Box<Player>,
        Some('2') => Box::new(AlphaBetaSearchPlayer::new(28)) as Box<Player>,
        Some('3') => Box::new(RandomPlayer::new(28)) as Box<Player>,
        _ => Box::new(HumanPlayer::new()) as Box<Player>,
    };

    print!("Second player? Type either key in [123] (AI by default): ");
    stdout().flush().unwrap();
    let white = match read_one_char() {
        Some('1') => Box::new(HumanPlayer::new()) as Box<Player>,
        Some('2') => Box::new(AlphaBetaSearchPlayer::new(28)) as Box<Player>,
        Some('3') => Box::new(RandomPlayer::new(28)) as Box<Player>,
        _ => Box::new(AlphaBetaSearchPlayer::new(28)) as Box<Player>,
    };

    GameManager::new(black, white).playout();
}
