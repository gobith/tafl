mod tafl;
mod position;

use tafl::Tafl;


fn main() {
    let board = Tafl::new(9);
    println!("{}", board);
}
