use std::collections::HashSet;
use std::fmt;
struct Tafl {
    size: u8,
    castles: HashSet<Position>,
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct Position {
    x: u8,
    y: u8,
}

impl Position {
    fn new(x: u8, y: u8) -> Self {
        Self { x, y }
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}@{} ", self.x, self.y)
    }
}

impl Tafl {
    fn castle_tile(&self) -> char {
        '⬛'
    }
    fn tile(&self) -> char {
        '⬜'
    }
    fn new(size: u8) -> Self {
        let mut tafl = Self {
            size: size,
            castles: HashSet::new(),
        };

        tafl.castles.insert(Position::new(0, 0));
        tafl.castles.insert(Position::new(0, size - 1));
        tafl.castles.insert(Position::new(size - 1, 0));
        tafl.castles.insert(Position::new((size - 1) / 2, (size - 1) / 2));
        tafl.castles.insert(Position::new(size - 1, size - 1));
        tafl
    }
}

impl fmt::Display for Tafl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for x in 0..self.size {
            for y in 0..self.size {
                let position = Position::new(x, y);

                if self.castles.contains(&position) {
                    write!(f, "{}", self.castle_tile())?;
                } else {
                    write!(f, "{}", self.tile())?;
                }
            }
            write!(f, "{}", '\n')?;
        }

        Ok(())
    }
}
fn main() {
    let board = Tafl::new(9);
    println!("{}", board)
}
