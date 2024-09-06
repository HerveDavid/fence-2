use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

use crate::cell::{Cell, Region};

// Hexagonal Efficient Coordinate System
// https://en.wikipedia.org/wiki/Hexagonal_Efficient_Coordinate_System
pub trait Coord {
    fn x(&self) -> &i32;
    fn y(&self) -> &i32;
    fn z(&self) -> &i32;
}

impl Coord for (i32, i32, i32) {
    fn x(&self) -> &i32 {
        &self.0
    }
    fn y(&self) -> &i32 {
        &self.1
    }
    fn z(&self) -> &i32 {
        &self.2
    }
}

impl Coord for (&i32, &i32, &i32) {
    fn x(&self) -> &i32 {
        self.0
    }
    fn y(&self) -> &i32 {
        self.1
    }
    fn z(&self) -> &i32 {
        self.2
    }
}

impl<'a> Borrow<dyn Coord + 'a> for (i32, i32, i32) {
    fn borrow(&self) -> &(dyn Coord + 'a) {
        self
    }
}

impl Hash for dyn Coord + '_ {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x().hash(state);
        self.y().hash(state);
        self.z().hash(state);
    }
}

impl PartialEq for dyn Coord + '_ {
    fn eq(&self, other: &Self) -> bool {
        self.x() == other.x() && self.y() == other.y() && self.z() == other.z()
    }
}

impl Eq for dyn Coord + '_ {}

#[derive(Debug)]
pub struct Grid {
    cells: HashMap<(i32, i32, i32), Cell>,
}

impl Grid {
    pub fn new() -> Self {
        Grid {
            cells: HashMap::new(),
        }
    }

    pub fn generate(width: i32) -> Self {
        let mut grid = Grid::new();

        let seed = (0, 0, 0);
        grid.insert(0, 0, 0, Cell::Empty(Region::White));

        // Green
        grid.generate_region(
            (1, 0, -1),
            (-1, 1, 0),
            width,
            seed,
            Cell::Empty(Region::Green),
        );

        // Yellow
        grid.generate_region(
            (1, -1, 0),
            (0, 1, -1),
            width,
            seed,
            Cell::Empty(Region::Yellow),
        );

        // Orange
        grid.generate_region(
            (0, -1, 1),
            (1, 0, -1),
            width,
            seed,
            Cell::Empty(Region::Orange),
        );

        // Red
        grid.generate_region(
            (-1, 0, 1),
            (1, -1, 0),
            width,
            seed,
            Cell::Empty(Region::Red),
        );

        // Violet
        grid.generate_region(
            (-1, 1, 0),
            (0, -1, 1),
            width,
            seed,
            Cell::Empty(Region::Violet),
        );

        // Blue
        grid.generate_region(
            (0, 1, -1),
            (-1, 0, 1),
            width,
            seed,
            Cell::Empty(Region::Blue),
        );

        grid
    }

    pub fn get(&self, x: &i32, y: &i32, z: &i32) -> Option<&Cell> {
        self.cells.get(&(x, y, z) as &dyn Coord)
    }

    pub fn insert(&mut self, x: i32, y: i32, z: i32, cell: Cell) -> Option<Cell> {
        self.cells.insert((x, y, z), cell)
    }

    pub fn len(&self) -> usize {
        self.cells.len()
    }

    pub fn nearest_neighbors(&self, x: &i32, y: &i32, z: &i32) -> Vec<&Cell> {
        let mut neighbors = vec![];

        if let Some(neighbor) = self.get(x, &(y + 1), &(z - 1)) {
            neighbors.push(neighbor);
        }

        if let Some(neighbor) = self.get(&(x + 1), y, &(z - 1)) {
            neighbors.push(neighbor);
        }

        if let Some(neighbor) = self.get(&(x + 1), &(y - 1), z) {
            neighbors.push(neighbor);
        }

        if let Some(neighbor) = self.get(x, &(y - 1), &(z + 1)) {
            neighbors.push(neighbor);
        }

        if let Some(neighbor) = self.get(&(x - 1), y, &(z + 1)) {
            neighbors.push(neighbor);
        }

        if let Some(neighbor) = self.get(&(x - 1), &(y + 1), z) {
            neighbors.push(neighbor);
        }

        neighbors
    }

    fn generate_next_cell(seed: (&i32, &i32, &i32), dir: (&i32, &i32, &i32)) -> (i32, i32, i32) {
        (seed.0 + dir.0, seed.1 + dir.1, seed.2 + dir.2)
    }

    fn generate_region(
        &mut self,
        axe_1: (i32, i32, i32),
        axe_2: (i32, i32, i32),
        width: i32,
        seed: (i32, i32, i32),
        cell: Cell,
    ) {
        let mut cell_1 = seed;
        for i in 0..width {
            cell_1 = Self::generate_next_cell(
                (&cell_1.0, &cell_1.1, &cell_1.1),
                (&axe_1.0, &axe_1.1, &axe_1.2),
            );
            self.insert(
                cell_1.x().clone(),
                cell_1.y().clone(),
                cell_1.z().clone(),
                cell.clone(),
            );

            let mut cell_2 = cell_1;
            for _ in 0..i {
                cell_2 = Self::generate_next_cell(
                    (&cell_2.0, &cell_2.1, &cell_2.1),
                    (&axe_2.0, &axe_2.1, &axe_2.2),
                );
                self.insert(
                    cell_2.x().clone(),
                    cell_2.y().clone(),
                    cell_2.z().clone(),
                    cell.clone(),
                );
            }
        }
    }
}
