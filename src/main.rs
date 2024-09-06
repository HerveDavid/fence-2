use grid::Grid;

mod cell;
mod grid;

fn main() {
    let mut grid = Grid::generate(3);

    let n = grid.nearest_neighbors(&0, &0, &0);

    println!("{:?}", n);
}
