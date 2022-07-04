mod position;
mod tafl_old;
mod tests;
mod brandubh;
mod hnefatafl;
mod tafl;
mod minimax;

use tafl_old::Tafl;

fn main() {
    let board = Tafl::new(7);
    println!("{}", board);
}

