use std::ops::{Add, Sub};

use rand::Rng;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[allow(unused)]
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

    fn xy(&self) -> (i32, i32) {
        (self.x, self.y)
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

pub fn rand_pos_in_range(range: Position) -> Position {
    let mut rng = rand::thread_rng();
    Position::from(rng.gen_range(0..range.x), rng.gen_range(0..range.y))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let a = Position::new();
        assert_eq!(a.x, 0);
        assert_eq!(a.y, 0);
    }

    #[test]
    fn from() {
        let a = Position::from(4, -8);
        assert_eq!(a.x, 4);
        assert_eq!(a.y, -8);
    }

    #[test]
    fn from_tup() {
        let a = Position::from_tup((4, -8));
        assert_eq!(a.x, 4);
        assert_eq!(a.y, -8);
    }

    #[test]
    fn xy() {
        assert_eq!(Position::from(4, -8).xy(), (4, -8));
    }

    #[test]
    fn add() {
        let a = Position { x: 4, y: -8 };
        let b = Position { x: 8, y: 7 };
        let c = a + b;
        assert_eq!(c.x, 12);
        assert_eq!(c.y, -1);
    }

    #[test]
    fn sub() {
        let a = Position { x: 4, y: -8 };
        let b = Position { x: 8, y: 7 };
        let c = a - b;
        assert_eq!(c.x, -4);
        assert_eq!(c.y, -15);
    }
}
