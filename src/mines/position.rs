use std::ops::{Add, Sub};

#[derive(Copy, Clone, PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new() -> Self {
        Self::from(0, 0)
    }

    pub fn from(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn from_tup(pos: (i32, i32)) -> Self {
        Self { x: pos.0, y: pos.1 }
    }
}

impl Add for Position {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Position::from(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub for Position {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Position::from(self.x - rhs.x, self.y - rhs.y)
    }
}
