use std::cmp::Ordering;
use std::fmt;

#[derive(Debug)]
pub struct Tafl<const N: usize> {
    row_size: usize,
    history: Vec<State<N>>,
    state: State<N>,
}

#[derive(Clone, Copy, Debug)]
struct State<const N: usize> {
    side: Side,
    row_size: usize,
    board: [Tile; N],
}

pub fn hnefatafl() -> Tafl<121> {
    use Tile::*;
    let state = State {
        side: Side::Attacker,
        row_size: 11,
        board: [
            Castle, Empty, Empty, Attacker, Attacker, Attacker, Attacker, Attacker, Empty, Empty,
            Castle, Empty, Empty, Empty, Empty, Empty, Attacker, Empty, Empty, Empty, Empty, Empty,
            Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Attacker,
            Empty, Empty, Empty, Empty, Defender, Empty, Empty, Empty, Empty, Attacker, Attacker,
            Empty, Empty, Empty, Defender, Defender, Defender, Empty, Empty, Empty, Attacker,
            Attacker, Attacker, Empty, Defender, Defender, King, Defender, Defender, Empty,
            Attacker, Attacker, Attacker, Empty, Empty, Empty, Defender, Defender, Defender, Empty,
            Empty, Empty, Attacker, Attacker, Empty, Empty, Empty, Empty, Defender, Empty, Empty,
            Empty, Empty, Attacker, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty,
            Empty, Empty, Empty, Empty, Empty, Empty, Empty, Attacker, Empty, Empty, Empty, Empty,
            Empty, Castle, Empty, Empty, Attacker, Attacker, Attacker, Attacker, Attacker, Empty,
            Empty, Castle,
        ],
    };

    Tafl {
        row_size: 11,
        history: vec![state],
        state: state,
    }
}

pub fn brandubh() -> Tafl<49> {
    use Tile::*;
    let state = State {
        side: Side::Attacker,
        row_size: 7,
        board: [
            Castle, Empty, Attacker, Attacker, Attacker, Empty, Castle, Empty, Empty, Empty,
            Attacker, Empty, Empty, Empty, Attacker, Empty, Empty, Defender, Empty, Empty,
            Attacker, Attacker, Attacker, Defender, King, Defender, Attacker, Attacker, Attacker,
            Empty, Empty, Defender, Empty, Empty, Attacker, Empty, Empty, Empty, Attacker, Empty,
            Empty, Empty, Castle, Empty, Attacker, Attacker, Attacker, Empty, Castle,
        ],
    };
    Tafl {
        row_size: 7,
        history: vec![state],
        state: state,
    }
}

impl<const N: usize> fmt::Display for Tafl<N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for x in 0..self.row_size {
            for y in 0..self.row_size {
                let tile = self.state.board[x * self.row_size + y];

                let char = match tile {
                    Tile::Empty => '???',
                    Tile::Attacker => '????',
                    Tile::Defender => '???',
                    Tile::King => '???',
                    Tile::Castle => '???',
                    Tile::CastleWithKing => '???',
                };
                write!(f, "{} {}", char, '\t')?;
            }
            write!(f, "{} {} {}", '\n', '\n', '\n')?;
        }

        let side_string = match self.state.side {
            Side::Attacker => "Attacker",
            Side::Defender => "Defender",
        };
        write!(f, "Turn: {}", side_string)?;
        write!(f, "{}", '\n')?;

        Ok(())
    }
}

impl<const N: usize> Tafl<N> {
    pub fn move_piece(&mut self, start_idx: usize, end_idx: usize) -> () {
        match self.state.move_piece(start_idx, end_idx) {
            Ok(mut new_state) => {
                new_state.switch_side();
                self.history.push(self.state);
                self.state = new_state;
            }
            Err(error_string) => {
                println!("Error {}", error_string)
            }
        }
    }
}

impl<const N: usize> State<N> {
    // fn row_col(&self, index: usize) -> (usize, usize) {
    //     (index / size, index % self.row_size)
    // }

    fn switch_side(&mut self) -> () {
        self.side = match self.side {
            Side::Attacker => Side::Defender,
            Side::Defender => Side::Attacker,
        };
    }

    fn move_piece(&self, start_idx: usize, end_idx: usize) -> Result<State<N>, String> {
        let start_tile = self.board[start_idx];

        match self.side {
            Side::Attacker => match start_tile {
                Tile::Empty => Err("Nothing on start position".to_string()),
                Tile::Attacker => match self.validate_move(start_idx, end_idx) {
                    Ok(_) => Ok(self.next_state(start_idx, end_idx)),
                    Err(str) => Err("Illegal Move: ".to_string() + &str),
                },
                Tile::Defender => Err("Attacker cannot move Defender".to_string()),
                Tile::King => Err("Attacker cannot move King".to_string()),
                Tile::Castle => Err("Cannot move from Castle".to_string()),
                Tile::CastleWithKing => Err("Cannot move from Castle".to_string()),
            },
            Side::Defender => match start_tile {
                Tile::Empty => Err("Nothing on start position".to_string()),
                Tile::Attacker => Err("Defender cannot move Attacker".to_string()),
                Tile::Defender | Tile::King => match self.validate_move(start_idx, end_idx) {
                    Ok(_) => Ok(self.next_state(start_idx, end_idx)),
                    Err(_) => Err("Illegal Move".to_string()),
                },
                Tile::Castle => Err("Cannot move from Castle".to_string()),
                Tile::CastleWithKing => Err("Cannot move from Castle".to_string()),
            },
        }
    }

    fn validate_move(&self, start_idx: usize, end_idx: usize) -> Result<(), String> {
        if self.board[end_idx] != Tile::Empty {
            return Err("End Tile is occupied".to_string());
        };

        // Range<usize >;

        // 1..4

        // let (start_row, start_col) = self.row_col(start_idx);

        let start_row = start_idx / self.row_size;
        let end_row = end_idx / self.row_size;
        let start_column = start_idx % self.row_size;
        let end_column = end_idx % self.row_size;

        println!("Start_idx {}", start_idx);
        println!("End_idx {}", end_idx);
        println!("Start_row {}", start_row);
        println!("End_row {}", end_row);
        println!("Start_column {}", start_column);
        println!("End_column {}", end_column);

        match start_row.cmp(&end_row) {
            Ordering::Less => {
                // End row greater than Start row
                if start_column != end_column {
                    return Err("Start end End Should be on the same Row or Column".to_string());
                };
                for n in 1..end_row - start_row {
                    if self.board[start_idx + (n * self.row_size)] != Tile::Empty {
                        return Err("Cannot move piece through another piece".to_string());
                    };
                }
            }
            Ordering::Greater => {
                // Start row greater than End row
                if start_column != end_column {
                    return Err("Start end End Should be on the same Row or Column".to_string());
                };

                for n in 1..start_row - end_row {
                    if self.board[start_idx - (n * self.row_size)] != Tile::Empty {
                        return Err("Cannot move piece through another piece".to_string());
                    };
                }
            }
            Ordering::Equal => {
                if start_column < end_column {
                    for n in start_idx + 1..end_idx {
                        println!("Index from Start is {}", n);
                        if self.board[n] != Tile::Empty {
                            return Err("Cannot move piece through another piece".to_string());
                        };
                    }
                } else {
                    for n in end_idx + 1..start_idx {
                        println!("Index from End is {}", n);
                        if self.board[n] != Tile::Empty {
                            return Err("Cannot move piece throught another piece".to_string());
                        };
                    }
                }
            }
        }

        Ok(())
    }

    fn next_state(&self, start_idx: usize, end_idx: usize) -> State<N> {
        let start = self.board[start_idx];
        let mut clone = self.clone();
        clone.board[end_idx] = start;
        clone.board[start_idx] = Tile::Empty;

        let end_row = end_idx / self.row_size;

        //left
        let index = end_idx - 2;
        if index > 0 && index / self.row_size == end_row {
            if self.side.tile_is_same_side(clone.board[index])
                && self.side.tile_is_opposite_side(clone.board[index + 1])
            {
                clone.board[index + 1] = Tile::Empty;
            };
        };

        //right
        let index = end_idx + 2;

        if index < self.row_size * self.row_size && index / self.row_size == end_row {
            if self.side.tile_is_same_side(clone.board[index])
                && self.side.tile_is_opposite_side(clone.board[index - 1])
            {
                clone.board[index - 1] = Tile::Empty;
            }
        };

        //up
        let index = end_idx - (self.row_size * 2);
        if index > 0 {
            if self.side.tile_is_same_side(clone.board[index])
                && self
                    .side
                    .tile_is_opposite_side(clone.board[index + self.row_size])
            {
                clone.board[index + self.row_size] = Tile::Empty;
            };
        };

        //down

        let index = end_idx + (self.row_size * 2);
        if index < self.row_size * self.row_size {
            if self.side.tile_is_same_side(clone.board[index])
                && self
                    .side
                    .tile_is_opposite_side(clone.board[index - self.row_size])
            {
                clone.board[index - self.row_size] = Tile::Empty;
            };
        };

        clone
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]

pub enum Tile {
    Attacker,
    Defender,
    King,
    Empty,
    Castle,
    CastleWithKing,
}

impl Tile {
    pub fn is_empty(&self) -> bool {
        *self == Tile::Empty
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Side {
    Attacker,
    Defender,
}

impl Side {
    fn tile_is_same_side(&self, tile: Tile) -> bool {
        match tile {
            Tile::Attacker => *self == Side::Attacker,
            Tile::Defender => *self == Side::Defender,
            Tile::King => *self == Side::Defender,
            _ => false,
        }
    }

    fn tile_is_opposite_side(&self, tile: Tile) -> bool {
        !self.tile_is_same_side(tile)
    }
}
