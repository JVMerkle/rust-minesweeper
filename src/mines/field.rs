// SPDX-FileCopyrightText: 2022 Julian Merkle
// SPDX-License-Identifier: GPL-3.0-only

use std::fmt::{Display, Formatter};

#[derive(Copy, Clone)]
pub struct Empty {
    neighbours: u8,
}

impl Empty {
    pub fn new() -> Self {
        Empty { neighbours: 0 }
    }

    pub fn neighbours(&self) -> u8 {
        self.neighbours
    }

    pub fn set_neighbours(&mut self, neighbours: u8) {
        self.neighbours = neighbours
    }
}

impl Display for Empty {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.neighbours != 0 {
            write!(f, "{}", self.neighbours)?;
        } else {
            write!(f, "░")?;
        }
        Ok(())
    }
}

#[derive(Copy, Clone)]
pub enum Type {
    Empty(Empty),
    Mine,
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match *self {
            Type::Empty(t) => write!(f, "{}", t),
            Type::Mine => write!(f, "⬤"),
        }
    }
}

pub struct Field {
    t: Type,
    hidden: bool,
    marked: bool,
}

impl Field {
    pub fn new_empty() -> Self {
        Field {
            t: Type::Empty(Empty::new()),
            hidden: true,
            marked: false,
        }
    }

    pub fn new_mine() -> Self {
        Field {
            t: Type::Mine,
            hidden: true,
            marked: false,
        }
    }

    pub fn typ(&self) -> Type {
        self.t
    }

    pub fn typ_mut(&mut self) -> &mut Type {
        &mut self.t
    }

    pub fn hidden(&self) -> bool {
        self.hidden
    }

    pub fn is_marked(&self) -> bool {
        self.marked
    }

    pub fn reveal(&mut self) {
        self.hidden = false;
    }

    pub fn mark(&mut self) {
        self.marked = true;
    }

    pub fn unmark(&mut self) {
        self.marked = false;
    }
}

impl Display for Field {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.hidden && self.marked {
            write!(f, "X")?;
        } else if self.hidden {
            write!(f, "▓")?;
        } else {
            write!(f, "{}", &self.t)?;
        }
        Ok(())
    }
}
