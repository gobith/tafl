use std::fmt;

#[derive(Hash, Eq, PartialEq, Debug)]
pub struct Position {
    x: u8,
    y: u8,
}

impl Position {
    pub fn new(x: u8, y: u8) -> Self {
        Self { x, y }
    }

    pub fn from(int: i32) -> Self {
        Self::new((int / 7).try_into().unwrap() , (int % 7).try_into().unwrap())
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        write!(f , "{}" , self.x * 7 + self.y)
        //write!(f, "{}@{} ", self.x, self.y)
    }
}
