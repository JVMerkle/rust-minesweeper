#![feature(generic_associated_types)]
#![allow(unused)]

extern crate core;

mod another;
mod event;
mod semver2;
mod thread_pool;
mod mines;

use mines::*;

fn main() {
    let mut field = Minesweeper::new(10, 10);

    for _ in 0..10 {
        let pos = rand_pos_in_range(Position::from(10, 10));
        field.set_cursor(pos);
        field.click();

        print!("{}", field);
    }
}
