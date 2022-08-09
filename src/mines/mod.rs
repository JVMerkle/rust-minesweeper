use std::array::IntoIter;
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};

use field::*;
use neighbour_iter::NeighbourIter;
use position::*;

pub mod position;
mod neighbour_iter;
mod field;

pub struct Minesweeper {
    fields: Vec<Field>,
    width: i32,
    height: i32,

    cursor: Position,

    initialized: bool,
    mine_count: usize,
    remaining_fields_to_open: usize,
}

pub enum GameStatus {
    MoreFieldsNeeded,
    YouWin,
    YouLose,
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Minesweeper {
    pub fn new(width: u32, height: u32) -> Minesweeper {
        let field_count = (width * height);

        let mut fields = Vec::new();
        fields.reserve(field_count as usize);

        for _ in 0..field_count {
            fields.push(Field::new_empty());
        }

        Minesweeper {
            fields,
            width: width as i32,
            height: height as i32,
            initialized: false,
            cursor: Position::from((width / 2) as i32, (height / 2) as i32),
            mine_count: 20 * (width * height) as usize / 100,
            remaining_fields_to_open: (width * height) as usize,
        }
    }

    pub fn move_cursor(&mut self, direction: Direction) {
        let vertical_jump = Position::from(0, 1);
        let horizontal_jump = Position::from(1, 0);
        let new_pos = match direction {
            Direction::Up => self.cursor - vertical_jump,
            Direction::Down => self.cursor + vertical_jump,
            Direction::Left => self.cursor - horizontal_jump,
            Direction::Right => self.cursor + horizontal_jump,
        };

        // Ignore errors
        let _ = self.set_cursor(new_pos);
    }

    pub fn set_cursor(&mut self, pos: Position) -> Result<(), ()> {
        if !self.is_valid_position(pos) {
            Err(())
        } else {
            self.cursor = pos;
            Ok(())
        }
    }

    pub fn toggle_marked_at_cursor(&mut self) {
        let mut field = self.field_at_mut(self.cursor).unwrap();
        if field.is_marked() {
            field.unmark();
        } else {
            field.mark();
        }
    }

    pub fn click(&mut self) -> GameStatus {
        if !self.initialized {
            self.initialized = true;

            self.fill_field_with_mines(self.mine_count as u32);
            self.remaining_fields_to_open = (self.width * self.height) as usize - self.mine_count;

            // First field is a bomb? That's bad, moving the bomb...
            // Fixme

            self.calc_neighbours_for_all_empty_fields();
        }

        let pos = self.cursor;
        let mut field = self.field_at_mut(pos).expect("Position not on the mines field!");

        // Prevent clicking marked and already opened fields
        if field.is_marked() || !field.hidden() {
            return GameStatus::MoreFieldsNeeded;
        }

        match field.typ_mut()
        {
            Type::Empty(_) => {
                field.reveal();
                self.remaining_fields_to_open -= 1;
                self.recursively_open_fields(pos);

                match self.remaining_fields_to_open {
                    0 => {
                        self.reveal_all_fields();
                        GameStatus::YouWin
                    }
                    _ => GameStatus::MoreFieldsNeeded,
                }
            }
            Type::Mine => {
                self.reveal_all_fields();
                GameStatus::YouLose
            }
        }
    }

    fn recursively_open_fields(&mut self, pos: Position) {
        for neighbour in NeighbourIter::new(pos).take(8) {
            if let Some(field) = self.field_at_mut(neighbour) {
                if let Type::Empty(empty_field) = field.typ() {
                    // Only process hidden fields
                    if field.hidden() {
                        field.reveal();
                        self.remaining_fields_to_open -= 1;
                        // Recursively open all fields that have ZERO neighbours
                        if empty_field.neighbours() == 0 {
                            self.recursively_open_fields(neighbour);
                        }
                    }
                }
            }
        }
    }

    fn reveal_all_fields(&mut self) {
        for field in self.fields.iter_mut() {
            field.reveal();
        }
    }

    fn fill_field_with_mines(&mut self, count: u32) {
        for _ in 0..count {
            let mine_pos = rand_pos_in_range(Position::from(self.width, self.height));

            match self.field_at_mut(mine_pos) {
                Some(field) => *field = Field::new_mine(),
                None => {}
            }
        }
    }

    fn calc_neighbours_for_all_empty_fields(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let position = Position::from(x, y);
                let count = self.get_neighbour_mines(position);
                match self.field_at_mut(position).unwrap().typ_mut() {
                    Type::Empty(e) => e.set_neighbours(count),
                    _ => {}
                }
            }
        }
    }

    fn iterate_over_neighbours(&mut self, pos: Position, mut cb: impl FnMut(Position, &mut Type)) {
        for neighbour in NeighbourIter::new(pos).take(8) {
            if let Some(neighbour_field) = self.field_at_mut(neighbour) {
                cb(neighbour, neighbour_field.typ_mut());
            }
        }
    }

    fn get_neighbour_mines(&mut self, pos: Position) -> u8 {
        let mut mine_count = 0;
        self.iterate_over_neighbours(pos, |_, field| {
            match field {
                Type::Mine => mine_count += 1,
                _ => {}
            };
        });
        mine_count as u8
    }

    fn field_at(&self, pos: Position) -> Option<&Field> {
        if self.is_valid_position(pos) {
            let idx = self.position_to_index(pos);
            return Some(&self.fields[idx]);
        }
        None
    }

    fn field_at_mut(&mut self, pos: Position) -> Option<&mut Field> {
        if self.is_valid_position(pos) {
            let idx = self.position_to_index(pos);
            return Some(&mut self.fields[idx]);
        }
        None
    }

    fn position_to_index(&self, pos: Position) -> usize {
        // index = y * width + x
        (pos.y * self.width + pos.x) as usize
    }

    fn index_to_position(&self, index: usize) -> Position {
        let x = index % self.width as usize;
        let y = index / self.width as usize;
        Position::from(x as i32, y as i32)
    }

    fn is_valid_position(&self, pos: Position) -> bool {
        if 0 <= pos.x && pos.x < self.width {
            if 0 <= pos.y && pos.y < self.height {
                return true;
            }
        }
        false
    }

    fn print_horizontal_border(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for _ in 0..(self.width + 1) {
            write!(f, " _ ")?;
        }
        write!(f, "\n")
    }
}

impl Display for Minesweeper {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Remaining fields: {}  Mines: {}\n", self.remaining_fields_to_open, self.mine_count)?;
        self.print_horizontal_border(f)?;

        for (i, field) in self.fields.iter().enumerate() {
            if (i) % (self.width as usize) == 0 {
                write!(f, "|")?;
            }

            if self.index_to_position(i) == self.cursor {
                write!(f, "[{}]", field)?;
            } else {
                write!(f, " {} ", field)?;
            }

            if (i + 1) % (self.width as usize) == 0 {
                write!(f, "|\n")?;
            }
        }
        self.print_horizontal_border(f)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke() {
        let mut mw = Minesweeper::new(10, 10);
        print!("{}", mw);

        for _ in 0..10 {
            let pos = rand_pos_in_range(Position::from(10, 10));
            mw.set_cursor(pos);
            mw.click();
            print!("{}", mw);
        }
    }
}
