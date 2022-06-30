mod position;
mod tafl_old;
mod tests;
mod brandubh;
mod hnefatafl;
mod tafl;

use tafl_old::Tafl;

fn main() {
    let board = Tafl::new(7);
    println!("{}", board);
}

