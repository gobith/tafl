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
    side: usize,
    row_size: usize,
    board: [u32; N],
}

pub fn hnefatafl() -> Tafl<121> {
    let state = State {
        side: 1,
        row_size: 11,
        board: [
            4, 0, 0, 1, 1, 1, 1, 1, 0, 0, 4, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 1, 0, 0, 0, 0, 2, 0, 0, 0, 0, 1, 1, 0, 0, 0, 2, 2, 2, 0, 0, 0, 1, 1, 1, 0,
            2, 2, 3, 2, 2, 0, 1, 1, 1, 0, 0, 0, 2, 2, 2, 0, 0, 0, 1, 1, 0, 0, 0, 0, 2, 0, 0, 0, 0,
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 4, 0, 0, 1, 1, 1,
            1, 1, 0, 0, 4,
        ],
    };

    Tafl {
        row_size: 11,
        history: vec![state],
        state: state,
    }
}

pub fn brandubh() -> Tafl<49> {
    let state = State {
        side: 1,
        row_size: 7,
        board: [
            4, 0, 1, 1, 1, 0, 4, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 2, 0, 0, 1, 1, 1, 2, 3, 2, 1, 1, 1,
            0, 0, 2, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 4, 0, 1, 1, 1, 0, 4,
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
                let num = self.state.board[x * self.row_size + y];

                let char = match num {
                    0 => '⬜',
                    1 => '🔴',
                    2 => '⭕',
                    3 => '⬤',
                    4 => '⬛',
                    _ => ' ',
                };
                write!(f, "{} {}", char, '\t')?;
            }
            write!(f, "{} {} {}", '\n', '\n', '\n')?;
        }

        let side_string = match self.state.side {
            1 => "Attacker",
            2 => "Defender",
            _ => "",
        };
        write!(f, "Turn: {}", side_string)?;
        write!(f, "{}", '\n')?;

        Ok(())
    }
}

impl<const N: usize> Tafl<N> {
    pub fn move_piece(&mut self, start_idx: usize, end_idx: usize) -> () {
        match self.state.move_piece(start_idx, end_idx) {
            StateResult::ValidState(mut new_state) => {
                new_state.switch_side();
                self.history.push(self.state);
                self.state = new_state;
            }
            StateResult::ErrorState(error_string) => {
                println!("Error {}", error_string)
            }
        }
    }
}

impl<const N: usize> State<N> {
    fn switch_side(&mut self) -> () {
        match self.side {
            1 => self.side = 2,
            2 => self.side = 1,
            _ => panic!("Wrong side"),
        }
    }

    fn validate_move(&self, start_idx: usize, end_idx: usize) -> Result<(), String> {
        if self.board[end_idx] != 0 {
            return Err("End Tile is occupied".to_string());
        };

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
                    if self.board[start_idx + (n * self.row_size)] != 0 {
                        return Err("Cannot move piece throught another piece".to_string());
                    };
                }
            }
            Ordering::Greater => {
                // Start row greater than End row
                if start_column != end_column {
                    return Err("Start end End Should be on the same Row or Column".to_string());
                };

                for n in 1..start_row - end_row {
                    if self.board[start_idx - (n * self.row_size)] != 0 {
                        return Err("Cannot move piece throught another piece".to_string());
                    };
                }
            }
            Ordering::Equal => {
                if start_idx < end_idx {
                    for n in start_idx + 1..end_idx {
                        println!("Index from Start is {}", n);
                        if self.board[n] != 0 {
                            return Err("Cannot move piece throught another piece".to_string());
                        };
                    }
                } else {
                    for n in end_idx + 1..start_idx {
                        println!("Index from End is {}", n);
                        if self.board[n] != 0 {
                            return Err("Cannot move piece throught another piece".to_string());
                        };
                    }
                }
            }
        }

        Ok(())
    }

    fn move_piece(&self, start_idx: usize, end_idx: usize) -> StateResult<State<N>> {
        let start = self.board[start_idx];
        // let end = self.board[end_idx];

        let mut clone = self.clone();
        clone.board[end_idx] = start;
        clone.board[start_idx] = 0;

        if self.side == 1 {
            match start {
                // Attacker
                0 => StateResult::ErrorState("Nothing on start position".to_string()),
                1 => match self.validate_move(start_idx, end_idx) {
                    Ok(_) => StateResult::ValidState(clone),
                    Err(str) => StateResult::ErrorState("Illegal Move: ".to_string() + &str),
                },
                2 => StateResult::ErrorState("Attacker cannot move Defender".to_string()),
                3 => StateResult::ErrorState("Attacker cannot move King".to_string()),
                4 => StateResult::ErrorState("Cannot move from Castle".to_string()),
                _ => panic!("No such number"),
            }
        } else {
            match start {
                // Defender
                0 => StateResult::ErrorState("Nothing on start position".to_string()),
                1 => StateResult::ErrorState("Defender cannot move Attacker".to_string()),
                2 => match self.validate_move(start_idx, end_idx) {
                    Ok(_) => StateResult::ValidState(clone),
                    Err(_) => StateResult::ErrorState("Illegal Move".to_string()),
                },
                3 => match self.validate_move(start_idx, end_idx) {
                    Ok(_) => StateResult::ValidState(clone),
                    Err(_) => StateResult::ErrorState("Illegal Move".to_string()),
                },
                4 => StateResult::ErrorState("Cannot move from Castle".to_string()),
                _ => panic!("No such number"),
            }
        }
    }
}

enum StateResult<T> {
    ValidState(T),
    ErrorState(String),
}
