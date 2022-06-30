use std::fmt;

#[derive(Clone , Debug)]
pub struct Tafl<const N: usize> {
    size: usize,
    board: [u8; N],
}

pub fn hnefatafl() -> Tafl<121> {
    Tafl {
        size: 11,
        board: [
            4, 0, 0, 3, 3, 3, 3, 3, 0, 0, 4, 
            0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
            3, 0, 0, 0, 0, 2, 0, 0, 0, 0, 3, 
            3, 0, 0, 0, 2, 2, 2, 0, 0, 0, 3, 
            3, 3, 0, 2, 2, 1, 2, 2, 0, 3, 3, 
            3, 0, 0, 0, 2, 2, 2, 0, 0, 0, 3, 
            3, 0, 0, 0, 0, 2, 0, 0, 0, 0, 3, 
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
            0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 
            4, 0, 0, 3, 3, 3, 3, 3, 0, 0, 4,
        ],
    }
}

pub fn brandubh() -> Tafl<49> {
    Tafl {
        size: 7,
        board: [
            4, 0, 3, 3, 3, 0, 4, 
            0, 0, 0, 3, 0, 0, 0, 
            3, 0, 0, 2, 0, 0, 3, 
            3, 3, 2, 1, 2, 3, 3, 
            3, 0, 0, 2, 0, 0, 3, 
            0, 0, 0, 3, 0, 0, 0, 
            4, 0, 3, 3, 3, 0, 4,
        ],
    }
}

impl<const N: usize> fmt::Display for Tafl<N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for x in 0..self.size {
            for y in 0..self.size {
                let num = self.board[x * self.size + y];

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

        Ok(())
    }
}
