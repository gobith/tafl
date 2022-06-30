use std::fmt;

#[derive(Debug)]
pub struct Tafl<const N: usize> {
    row_size: usize,
    history: Vec<State<N>>,
    state: State<N>,
}
#[derive(Clone, Copy , Debug)]
struct State<const N: usize> {
    side: usize,
    board: [u32; N],
}

pub fn hnefatafl() -> Tafl<121> {
    Tafl {
        row_size: 11,
        history: Vec::new(),
        state: State {
            side: 1,
            board: [
                4, 0, 0, 1, 1, 1, 1, 1, 0, 0, 4, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 2, 0, 0, 0, 0, 1, 1, 0, 0, 0, 2, 2, 2, 0, 0, 0, 1, 1,
                1, 0, 2, 2, 3, 2, 2, 0, 1, 1, 1, 0, 0, 0, 2, 2, 2, 0, 0, 0, 1, 1, 0, 0, 0, 0, 2, 0,
                0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 4, 0,
                0, 1, 1, 1, 1, 1, 0, 0, 4,
            ],
        },
    }
}

pub fn brandubh() -> Tafl<49> {
    Tafl {
        row_size: 7,
        history: Vec::new(),
        state: State {
            side: 1,
            board: [
                4, 0, 1, 1, 1, 0, 4, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 2, 0, 0, 1, 1, 1, 2, 3, 2, 1, 1,
                1, 0, 0, 2, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 4, 0, 1, 1, 1, 0, 4,
            ],
        },
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
        let new_state = self.state.move_piece(start_idx, end_idx);
        self.history.push(self.state);
        self.state = new_state;
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

    fn move_piece(&self, start_idx: usize, end_idx: usize) -> State<N> {
        let mut clone = self.clone();

        let start = clone.board[start_idx];
        let end = clone.board[end_idx];

        clone.board[end_idx] = start;
        clone.board[start_idx] = 0;

        clone.switch_side();
        clone
    }
}
