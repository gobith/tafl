use std::fmt;

#[derive(Debug)]
pub struct Brandubh {
    size: usize,
    board: [u32; 49],
}

impl Brandubh {
    pub fn new() -> Self {
        Self {
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
}

impl fmt::Display for Brandubh {
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
