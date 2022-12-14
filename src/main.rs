// SPDX-FileCopyrightText: 2022 Julian Merkle
// SPDX-License-Identifier: GPL-3.0-only

use std::io::Read;

use mines::*;

mod mines;
mod term;

fn main() {
    let term = term::FastTerm::new();

    let mut stdin_iter = std::io::stdin().bytes();

    let mut field = Minesweeper::new(10, 10);

    term.clear();

    loop {
        print!("{}", field);

        let byte = match stdin_iter.next() {
            Some(opt) => opt.unwrap(),
            None => break,
        };

        term.clear();

        match byte {
            b'w' => field.move_cursor(Direction::Up),
            b's' => field.move_cursor(Direction::Down),
            b'a' => field.move_cursor(Direction::Left),
            b'd' => field.move_cursor(Direction::Right),
            b'f' => field.toggle_marked_at_cursor(),
            b' ' => match field.click() {
                GameStatus::YouWin => {
                    println!("{}\nYou win! :-)", field);
                    break;
                }
                GameStatus::YouLose => {
                    println!("{}\nYou lose :-(", field);
                    break;
                }
                GameStatus::Continue => {}
            },
            _ => { /* Key not mapped */ }
        };
    }
}
