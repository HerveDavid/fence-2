use coord::{Coord, Near};
use dir::{Dir, DIR};
use grid::Grid;

mod cell;
mod coord;
mod dir;
mod grid;

fn main() {
    let cell = (0, 0, 0);

    let dir = &DIR[0].apply(&cell);

    println!("{dir:?}");

    // let dir = Dir::<(i32, i32, i32)>::SW(|&(a, r, c)| (1 - a, r + a, c + a));
    // match dir {
    //     Dir::SW(f) => f(&cell),
    //     _ => unreachable!(),
    // };

    for neighbor in Dir::nearest_neighbors() {
        let i = neighbor.apply(&cell);
        println!("{:?}", i);
    }
}
