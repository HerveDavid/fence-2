use crate::{coord::Near, dir::Dir};
use std::{cell, collections::HashMap, fmt::Debug, i32, rc::Rc};

use crate::coord::Coord;

#[derive(Debug)]
pub struct Grid {
    array_0: HashMap<(i32, i32), Rc<f64>>,
    array_1: HashMap<(i32, i32), Rc<f64>>,
}

impl Grid {
    pub fn new() -> Self {
        Self {
            array_0: HashMap::new(),
            array_1: HashMap::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.array_0.len() + self.array_1.len()
    }

    pub fn get<C: Coord>(&self, coord: &C) -> Option<&f64> {
        let a = coord.a();
        let r = coord.r();
        let c = coord.c();
        match a {
            0 => self.array_0.get(&(*r, *c)).map(Rc::as_ref),
            1 => self.array_1.get(&(*r, *c)).map(Rc::as_ref),
            _ => panic!("only 2 arrays"),
        }
    }

    pub fn insert<C: Coord>(&mut self, coord: C, cell: f64) -> Option<f64> {
        let a = coord.a();
        let r = coord.r();
        let c = coord.c();

        let cell = Rc::new(cell);

        let existing = match a {
            0 => self.array_0.insert((*r, *c), cell.clone()),
            1 => self.array_1.insert((*r, *c), cell.clone()),
            _ => panic!("only 2 arrays"),
        };

        existing.map(|e| Rc::into_inner(e).unwrap())
    }

    pub fn create_region<C: Coord>(
        &self,
        axe_a: &C,
        axe_b: &C,
        width: i32,
        seed: &C,
    ) -> Vec<impl Coord + Debug> {
        let mut coords: Vec<(i32, i32, i32)> = vec![];

        let mut cell = (*seed.a(), *seed.r(), *seed.c());
        for i in 0..width {
            cell = cell.near(axe_a);
            coords.push(cell.clone());

            let mut cell_1 = cell;
            for _ in 0..i {
                cell_1 = cell_1.near(axe_b);
                coords.push(cell_1.clone());
            }
        }

        coords
    }

    pub fn nearest_neighbords<C: Coord>(&self, coord: &C) -> Vec<impl Coord + Debug> {
        let a = coord.a();
        let r = coord.r();
        let c = coord.c();

        let mut neighbords: Vec<(i32, i32, i32)> = vec![];

        for dir in Dir::iter() {
            let coord = dir.apply(&(*a, *r, *c));
            if self.get(&coord).is_some() {
                println!("{coord:?}");
                neighbords.push(coord);
            }
        }

        neighbords
    }

    // pub fn generate(width: i32) -> Self {
    //     let mut grid = Grid::new();

    //     let seed = (0, 0, 0);
    //     grid.insert(seed, Cell::Empty(Region::White));

    //     // Green
    //     grid.generate_region(
    //         (1, 0, -1),
    //         (-1, 1, 0),
    //         width,
    //         seed,
    //         Cell::Empty(Region::Green),
    //     );

    //     // // Yellow
    // grid.generate_region(
    //     (1, -1, 0),
    //     (0, 1, -1),
    //     width,
    //     seed,
    //     Cell::Empty(Region::Yellow),
    // );

    // // Orange
    // grid.generate_region(
    //     (0, -1, 1),
    //     (1, 0, -1),
    //     width,
    //     seed,
    //     Cell::Empty(Region::Orange),
    // );

    // // Red
    // grid.generate_region(
    //     (-1, 0, 1),
    //     (1, -1, 0),
    //     width,
    //     seed,
    //     Cell::Empty(Region::Red),
    // );

    // // Violet
    // grid.generate_region(
    //     (-1, 1, 0),
    //     (0, -1, 1),
    //     width,
    //     seed,
    //     Cell::Empty(Region::Violet),
    // );

    // // Blue
    // grid.generate_region(
    //     (0, 1, -1),
    //     (-1, 0, 1),
    //     width,
    //     seed,
    //     Cell::Empty(Region::Blue),
    // );

    //     grid
    // }

    // pub fn get(&self, coord: &(i32, i32, i32)) -> Option<&Cell> {
    //     self.cells.get(coord)
    // }

    // pub fn insert(&mut self, coord: (i32, i32, i32), cell: Cell) -> Option<Cell> {
    //     self.cells.insert(coord, cell)
    // }

    // pub fn len(&self) -> usize {
    //     self.cells.len()
    // }

    // pub fn nearest_neighbors(&self, x: &i32, y: &i32, z: &i32) -> Vec<&Cell> {
    //     let mut neighbors = vec![];

    //     if let Some(neighbor) = self.get(x, &(y + 1), &(z - 1)) {
    //         neighbors.push(neighbor);
    //     }

    //     if let Some(neighbor) = self.get(&(x + 1), y, &(z - 1)) {
    //         neighbors.push(neighbor);
    //     }

    //     if let Some(neighbor) = self.get(&(x + 1), &(y - 1), z) {
    //         neighbors.push(neighbor);
    //     }

    //     if let Some(neighbor) = self.get(x, &(y - 1), &(z + 1)) {
    //         neighbors.push(neighbor);
    //     }

    //     if let Some(neighbor) = self.get(&(x - 1), y, &(z + 1)) {
    //         neighbors.push(neighbor);
    //     }

    //     if let Some(neighbor) = self.get(&(x - 1), &(y + 1), z) {
    //         neighbors.push(neighbor);
    //     }

    //     neighbors
    // }

    // fn generate_next_cell(seed: &(i32, i32, i32), dir: &(i32, i32, i32)) -> (i32, i32, i32) {
    //     (seed.0 + dir.0, seed.1 + dir.1, seed.2 + dir.2)
    // }

    // fn generate_region(
    //     &mut self,
    //     axe_1: (i32, i32, i32),
    //     axe_2: (i32, i32, i32),
    //     width: i32,
    //     seed: (i32, i32, i32),
    //     cell: Cell,
    // ) {
    //     let mut coord_a = seed;
    //     for i in 0..width {
    //         coord_a = Self::generate_next_cell(&coord_a, &axe_1);
    //         self.insert(coord_a, cell.clone());

    //         let mut coord_b = coord_a;
    //         for _ in 0..i {
    //             coord_b = Self::generate_next_cell(&coord_b, &axe_2);
    //             self.insert(coord_b, cell.clone());
    //         }
    //     }
    // }
}
