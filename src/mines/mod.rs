use std::array::IntoIter;
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};

use rand::Rng;

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

pub fn rand_pos_in_range(range: Position) -> Position {
    let mut rng = rand::thread_rng();
    Position::from(rng.gen_range(0..range.x), rng.gen_range(0..range.y))
}

#[derive(Copy, Clone)]
struct Empty {
    neighbours: u8,
}

impl Empty {
    fn new() -> Self {
        Empty { neighbours: 0 }
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
enum Type {
    Empty(Empty),
    Mine,
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match *self {
            Type::Empty(t) => write!(f, "{}", t),
            Type::Mine => write!(f, "X"),
        }
    }
}

struct Field {
    t: Type,
    hidden: bool,
}

impl Field {
    fn new_empty() -> Self {
        Field {
            t: Type::Empty(Empty::new()),
            hidden: true,
        }
    }

    fn new_mine() -> Self {
        Field {
            t: Type::Mine,
            hidden: true,
        }
    }

    fn typ(&self) -> Type {
        self.t
    }

    fn typ_mut(&mut self) -> &mut Type {
        &mut self.t
    }

    fn is_hidden(&self) -> bool {
        self.hidden
    }

    fn reveal(&mut self) {
        self.hidden = false;
    }
}

impl Display for Field {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.hidden {
            write!(f, "▓")?;
        } else {
            write!(f, "{}", &self.t)?;
        }
        Ok(())
    }
}

pub struct Minesweeper {
    fields: Vec<Field>,
    width: i32,
    height: i32,

    cursor: Position,

    initialized: bool,
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
        }
    }

    // pub fn cursor_up(&mut self) {
    //     let new_pos =
    //     self.is_valid_position()
    //     if self.cursor.0 >= self.width {
    //         self.cursor.0 -= self.width;
    //     }
    // }
    //
    // pub fn cursor_down(&mut self) {
    //     if self.cursor.0 < self. {
    //         self.cursor.0 -= self.width;
    //     }
    // }

    pub fn set_cursor(&mut self, pos: Position) {
        if !self.is_valid_position(pos) {
            panic!("Position not on the mines field!");
        }

        self.cursor = pos;
    }

    pub fn click(&mut self) {
        if !self.initialized {
            self.initialized = true;

            let mine_count = 20 * (self.width * self.height) / 100;
            self.fill_field_with_mines(mine_count as u32);

            // First field is a bomb? That's bad, moving the bomb...
            // Fixme

            self.calc_neighbours_for_all_empty_fields();
        }

        let pos = self.cursor;
        let mut field = self.field_at_mut(pos).expect("Position not on the mines field!");
        match field.typ_mut()
        {
            Type::Empty(_) => {
                println!("Nice!");
                field.reveal();
                self.recursively_open_fields(pos);
            }
            Type::Mine => {
                println!("You lose!");
                self.reveal_all_fields();
            }
        }
    }

    fn recursively_open_fields(&mut self, pos: Position) {
        for neighbour in NeighbourIter::new(pos).take(8) {
            if let Some(field) = self.field_at_mut(neighbour) {
                if let Type::Empty(empty_field) = field.typ() {
                    // Only process hidden fields
                    if field.is_hidden() {
                        field.reveal();
                        // Recursively open all fields that have ZERO neighbours
                        if empty_field.neighbours == 0 {
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
                    Type::Empty(e) => e.neighbours = count,
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

struct NeighbourIter {
    origin: Position,
    i: usize,
    distance: i32,
}

impl NeighbourIter {
    fn new(origin: Position) -> Self {
        NeighbourIter {
            origin,
            i: 0,
            distance: 1,
        }
    }
}

impl Iterator for NeighbourIter {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        let neighbour_offsets = [
            (-1, -1), (0, -1), (1, -1),
            (-1, 0), /*******/ (1, 0),
            (-1, 1), (0, 1), (1, 1),
        ];

        let offset = neighbour_offsets[self.i];

        let pos = Position::from(self.origin.x + offset.0 * self.distance, self.origin.y + offset.1 * self.distance);

        self.i += 1;
        if self.i >= neighbour_offsets.len() {
            self.i = 0;
            self.distance += 1;
        }

        Some(pos)
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::E;

    use super::*;

    #[test]
    fn aaa() {
        let mut mw = Minesweeper::new(10, 10);
        print!("{}", mw);

        for _ in 0..10 {
            let pos = rand_pos_in_range(Position::from(10, 10));
            mw.set_cursor(pos);
            mw.click();
            print!("{}", mw);
        }
    }


    #[test]
    fn neighbour_iter_unique_values() {
        let neighbours: Vec<Position> = NeighbourIter::new(Position::from(0, 0)).take(20).collect();
        let mut unique = neighbours.clone();
        unique.sort();
        unique.dedup();

        assert_eq!(neighbours.len(), 20);
        assert_eq!(neighbours.len(), unique.len());
    }
}
