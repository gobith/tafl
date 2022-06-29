mod position;
mod tafl;
mod tests;
mod brandubh;
mod hnefatafl;

use tafl::Tafl;

fn main() {
    let board = Tafl::new(7);
    println!("{}", board);
}

