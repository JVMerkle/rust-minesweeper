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
        } else if self.neighbours == 0 {
            write!(f, "░")?;
        } else {
            write!(f, "{}", self.neighbours)?;
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

            let mine_count = 15 * (self.width * self.height) / 100;
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
            }
            Field::Mine(m) => {
                println!("You lose!");
                m.hidden = false;
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

    fn get_neighbour_mines(&self, pos: Position) -> u8 {
        let neighbour_offsets = [
            (-1, -1), (0, -1), (1, -1),
            (-1, 0), (0, 0), (1, 0),
            (-1, 1), (0, 1), (1, 1),
        ];

        let mut mine_count = 0;
        for offset in neighbour_offsets {
            let x = pos.0 as i32 + offset.0;
            let y = pos.1 as i32 + offset.1;

            if x >= 0 && x < self.width {
                if y >= 0 && y < self.height {
                    match self.field_at((x, y)) {
                        Some(Field::Mine(_)) => mine_count += 1,
                        _ => {}
                    }
                }
            }
        }
        mine_count as u8
    }

    fn field_at(&self, pos: Position) -> Option<&Field> {
        let idx = (pos.1 * self.width + pos.0) as usize;

        if idx < self.fields.len() {
            Some(&self.fields[idx])
        } else {
            None
        }
    }

    fn field_at_mut(&mut self, pos: Position) -> Option<&mut Field> {
        let idx = (pos.1 * self.width + pos.0) as usize;

        if idx < self.fields.len() {
            Some(&mut self.fields[idx])
        } else {
            None
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aaa() {
        let mut mw = Minesweeper::new(10, 10);
        print!("{}", mw);

        for x in 0..5 {
            for y in 0..5 {
                mw.click((x, y));
                print!("{}", mw);
            }
        }
    }
}
