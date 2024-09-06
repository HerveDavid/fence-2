use coord::{Coord, Near};
use dir::{Dir, DIR};
use grid::Grid;

mod cell;
mod coord;
mod dir;
mod grid;

fn main() {
    let mut grid = Grid::new();
    grid.insert((0, 0, 0), 1.0);
    grid.insert((1, 0, 1), 2.0);

    let v = grid.nearest_neighbords(&(0, 0, 0));
    println!("{v:?}");
}
