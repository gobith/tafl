use std::fmt;

#[derive(Debug)]
pub struct Brandubh {
    size: usize,
    board: [u8; 49],
}

impl Brandubh {
    pub fn new() -> Self {
        Self {
            size: 7,
            board: [
                0, 0, 3, 3, 3, 0, 0, 0, 0, 0, 3, 0, 0, 0, 3, 0, 0, 2, 0, 0, 3, 3, 3, 2, 1, 2, 3, 3,
                3, 0, 0, 2, 0, 0, 3, 0, 0, 0, 3, 0, 0, 0, 0, 0, 3, 3, 3, 0, 0,
            ],
        }
    }
}

impl fmt::Display for Brandubh {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for x in 0..self.size {
            for y in 0..self.size {
                let num = self.board[x * self.size + y];

                match num {
                    0 => write!(f, "{} {}", '⬜', '\t')?,
                    1 => write!(f, "{} {}", '🔴', '\t')?,
                    2 => write!(f, "{} {}", '⭕', '\t')?,
                    3 => write!(f, "{} {}", '⬤', '\t')?,
                    _ => write!(f, "{} {}", "", '\t')?,
                };
            }
            write!(f, "{} {} {}", '\n', '\n', '\n')?;
        }

        Ok(())
    }
}
