use std::fmt;

#[derive(Debug)]
pub struct Hnefatafl {
    size: usize,
    board: [u8; 121],
}

impl Hnefatafl {
    pub fn new() -> Self {
        Self {
            size: 11,
            board: [
                4, 0, 0, 3, 3, 3, 3, 3, 0, 0, 4,
                0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3,
                3, 0, 0, 0, 2, 2, 2, 0, 0, 0, 3,
                3, 3, 0, 0, 2, 1, 2, 0, 0, 3, 3,
                3, 0, 0, 0, 2, 2, 2, 0, 0, 0, 3,
                3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0,
                4, 0, 0, 3, 3, 3, 3, 3, 0, 0, 4
            ],
        }
    }
}