use coord::{Coord, Near};
use grid::Grid;

mod cell;
mod coord;
mod grid;

fn main() {
    let mut grid = Grid::new();

    for c in grid.create_region(&(1, 1, 1), &(1, 1, 1), 3, &(0, 0, 0)) {
        println!("{c:?}");
    }
}
