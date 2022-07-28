use std::array::IntoIter;
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};

use rand::Rng;

type Position = (i32, i32);

fn rand_pos_in_range(range: Position) -> Position {
    let mut rng = rand::thread_rng();
    (rng.gen_range(0..range.0), rng.gen_range(0..range.1))
}

struct Empty {
    hidden: bool,
    neighbours: u8,
}

impl Empty {
    fn new() -> Self {
        Empty { hidden: true, neighbours: 0 }
    }

    fn show(&mut self) {
        self.hidden = true;
    }
}

impl Display for Empty {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.hidden {
            write!(f, "▓")?;
        } else if self.neighbours != 0 {
            write!(f, "{}", self.neighbours)?;
        } else {
            write!(f, "░")?;
        }
        Ok(())
    }
}

struct Mine {
    hidden: bool,
}

impl Mine {
    fn new() -> Self {
        Mine { hidden: true }
    }

    fn show(&mut self) {
        self.hidden = true;
    }
}

impl Display for Mine {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.hidden {
            write!(f, "▓")?;
        } else {
            write!(f, "X")?;
        }
        Ok(())
    }
}

enum Field {
    Empty(Empty),
    Mine(Mine),
}

struct Minesweeper {
    fields: Vec<Field>,
    width: i32,
    height: i32,

    initialized: bool,
}

impl Minesweeper {
    fn new(width: u32, height: u32) -> Minesweeper {
        let field_count = (width * height);

        let mut fields = Vec::new();
        fields.reserve(field_count as usize);

        for _ in 0..field_count {
            fields.push(Field::Empty(Empty::new()));
        }

        Minesweeper {
            fields,
            width: width as i32,
            height: height as i32,
            initialized: false,
        }
    }

    fn click(&mut self, pos: Position) {
        if !self.initialized {
            self.initialized = true;

            let mine_count = 20 * (self.width * self.height) / 100;
            self.fill_with_mines(mine_count as u32);

            // First field is a bomb? That's bad, moving the bomb...
            // Fixme

            self.calc_neighbour_mines();
        }

        match self.field_at_mut(pos).expect("Position not on the mines field!")
        {
            Field::Empty(e) => {
                println!("Nice!");
                e.hidden = false;
                self.recurse_open(pos);
            }
            Field::Mine(m) => {
                println!("You lose!");
                self.show_all_fields();
            }
        }
    }

    fn recurse_open(&mut self, pos: Position) {
        for neighbour in NeighbourIter::new(pos).take(8) {
            if let Some(Field::Empty(empty_field)) = self.field_at_mut(neighbour) {
                // Open all empty fields
                if empty_field.hidden {
                    empty_field.hidden = false;

                    // Recursively open all fields that have ZERO neighbours
                    if empty_field.neighbours == 0 {
                        self.recurse_open(neighbour);
                    }
                }
            }
        }
    }

    fn show_all_fields(&mut self) {
        for field in self.fields.iter_mut() {
            match field {
                Field::Empty(e) => e.hidden = false,
                Field::Mine(e) => e.hidden = false,
            }
        }
    }

    fn fill_with_mines(&mut self, count: u32) {
        for _ in 0..count {
            let mine_pos = rand_pos_in_range((self.width, self.height));

            match self.field_at_mut(mine_pos) {
                Some(field) => *field = Field::Mine(Mine::new()),
                None => {}
            }
        }
    }

    fn calc_neighbour_mines(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let position = (x, y);
                let count = self.get_neighbour_mines(position);
                match self.field_at_mut(position).unwrap() {
                    Field::Empty(e) => e.neighbours = count,
                    _ => {}
                }
            }
        }
    }

    fn iterate_over_neighbours(&mut self, pos: Position, mut cb: impl FnMut(Position, &mut Field)) {
        for neighbour in NeighbourIter::new(pos).take(8) {
            if let Some(neighbour_field) = self.field_at_mut(neighbour) {
                cb(neighbour, neighbour_field);
            }
        }
    }

    fn get_neighbour_mines(&mut self, pos: Position) -> u8 {
        let mut mine_count = 0;
        self.iterate_over_neighbours(pos, |_, field| {
            match field {
                Field::Mine(_) => mine_count += 1,
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
        (pos.1 * self.width + pos.0) as usize
    }

    fn is_valid_position(&self, pos: Position) -> bool {
        if 0 <= pos.0 && pos.0 < self.width {
            if 0 <= pos.1 && pos.1 < self.height {
                return true;
            }
        }
        false
    }

    fn print_border(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for _ in 0..(self.width + 1) {
            write!(f, " _ ")?;
        }
        write!(f, "\n")
    }
}

impl Display for Minesweeper {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.print_border(f)?;

        for (i, field) in self.fields.iter().enumerate() {
            if (i) % (self.width as usize) == 0 {
                write!(f, "|")?;
            }

            match field {
                Field::Mine(m) => write!(f, " {} ", m)?,
                Field::Empty(e) => write!(f, " {} ", e)?,
            }

            if (i + 1) % (self.width as usize) == 0 {
                write!(f, "|\n")?;
            }
        }
        self.print_border(f)?;
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

        let pos = (self.origin.0 + offset.0 * self.distance, self.origin.1 + offset.1 * self.distance);

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
            let pos = rand_pos_in_range((10, 10));
            mw.click(pos);
            print!("{}", mw);
        }
    }


    #[test]
    fn neighbour_iter_unique_values() {
        let neighbours: Vec<Position> = NeighbourIter::new((0, 0)).take(20).collect();
        let mut unique = neighbours.clone();
        unique.sort();
        unique.dedup();

        assert_eq!(neighbours.len(), 20);
        assert_eq!(neighbours.len(), unique.len());
    }
}
