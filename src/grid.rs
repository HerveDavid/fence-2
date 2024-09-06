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

    pub fn nearest_neighbords<C: Coord>(&self, coord: &C) -> Vec<&f64> {
        let a = coord.a();
        let r = coord.r();
        let c = coord.c();

        Dir::iter()
            .filter_map(|dir| self.get(&dir.apply(&(*a, *r, *c))))
            .collect()
    }
}
