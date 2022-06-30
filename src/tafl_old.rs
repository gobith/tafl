#[allow(dead_code)]
use super::position::Position;
use std::collections::HashSet;
use std::fmt;

#[derive(Debug)]
pub struct Tafl {
    size: u8,
    castles: HashSet<Position>,
    attackers: HashSet<Position>,
    defenders: HashSet<Position>,
    king: Position,
}

impl fmt::Display for Tafl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for x in 0..self.size {
            for y in 0..self.size {
                let position = Position::new(x, y);

                if self.defenders.contains(&position) {
                    write!(f, "{}", self.defender_piece())?;
                } else if self.attackers.contains(&position) {
                    write!(f, "{}", self.attacker_piece())?;
                } else if self.king == position {
                    write!(f, "{}", self.king_piece())?;
                } else if self.castles.contains(&position) {
                    write!(f, "{}", self.castle_tile())?;
                } else {
                    write!(f, "{}", self.tile())?;
                }
                // write!(f, "{} ", position)?;
                write!(f, "{}", '\t')?;
            }
            write!(f, "{} {} {}", '\n', '\n', '\n')?;
        }

        Ok(())
    }
}

impl Tafl {
    fn castle_tile(&self) -> char {
        '⬛'
    }
    fn tile(&self) -> char {
        '⬜'
    }
    fn king_piece(&self) -> char {
        '🔴'
    }
    fn defender_piece(&self) -> char {
        '⭕'
    }
    fn attacker_piece(&self) -> char {
        '⬤'
    }
    pub fn new(size: u8) -> Self {
        let mut tafl = Self {
            size: size,
            castles: HashSet::new(),
            attackers: HashSet::new(),
            defenders: HashSet::new(),
            king: Position::new((size - 1) / 2, (size - 1) / 2),
        };

        tafl.castles.insert(Position::new(0, 0));
        tafl.castles.insert(Position::new(0, size - 1));
        tafl.castles.insert(Position::new(size - 1, 0));
        tafl.castles
            .insert(Position::new((size - 1) / 2, (size - 1) / 2));
        tafl.castles.insert(Position::new(size - 1, size - 1));
        tafl
    }

    pub fn from(tfl: &str) -> Self {
        let mut v = tfl.split('+');
        let size: u8 = v.next().unwrap().parse().unwrap();
        let mut tafl = Self::new(size);

        tafl.king = Position::from(v.next().unwrap().parse().unwrap());

        let defender_iter = v.next().unwrap().split('-');
        for pos_str in defender_iter {
            tafl.defenders
                .insert(Position::from(pos_str.parse().unwrap()));
        }

        let attacker_iter = v.next().unwrap().split('-');
        for pos_str in attacker_iter {
            tafl.attackers
                .insert(Position::from(pos_str.parse().unwrap()));
        }

        tafl
    }

    pub fn brandubh() -> Self {
        Self::from("7+24+17-23-25-31+3-10-21-22-26-27-38-45")
    }

}
