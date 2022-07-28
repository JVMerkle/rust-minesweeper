#![feature(generic_associated_types)]
#![allow(unused)]

extern crate core;

use std::io::Read;

use mines::*;
use mines::position::*;

mod another;
mod event;
mod semver2;
mod thread_pool;
mod mines;

fn main() {
    let mut stdin_iter = std::io::stdin().bytes();

    let mut field = Minesweeper::new(10, 10);

    loop {
        print!("{}", field);

        let byte = match stdin_iter.next() {
            Some(opt) => opt.unwrap(),
            None => break,
        };

        //println!("Read {:?}", byte);

        match byte {
            b'w' => field.move_cursor(Direction::Up),
            b's' => field.move_cursor(Direction::Down),
            b'a' => field.move_cursor(Direction::Left),
            b'd' => field.move_cursor(Direction::Right),
            b'x' => field.toggle_marked_at_cursor(),
            b' ' => field.click(),
            _ => { continue; }
        };

        for _ in 0..15 {
            println!();
        }
    }
}
