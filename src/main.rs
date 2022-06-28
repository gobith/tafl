mod position;
mod tafl;
mod tests;

use tafl::Tafl;

fn main() {
    let board = Tafl::new(7);
    println!("{}", board);
}

