use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

// Hexagonal Efficient Coordinate System
// https://en.wikipedia.org/wiki/Hexagonal_Efficient_Coordinate_System
pub trait Coord<A, B> {
    fn row(&self) -> &A;
    fn col(&self) -> &B;
}

impl<'a, A, B> Borrow<dyn Coord<A, B> + 'a> for (A, B)
where
    A: Eq + Hash + 'a,
    B: Eq + Hash + 'a,
{
    fn borrow(&self) -> &(dyn Coord<A, B> + 'a) {
        self
    }
}

impl<A: Hash, B: Hash> Hash for dyn Coord<A, B> + '_ {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.row().hash(state);
        self.col().hash(state);
    }
}

impl<A: Eq, B: Eq> PartialEq for dyn Coord<A, B> + '_ {
    fn eq(&self, other: &Self) -> bool {
        self.row() == other.row() && self.col() == other.col()
    }
}

impl<A: Eq, B: Eq> Eq for dyn Coord<A, B> + '_ {}

pub struct Grid<A: Eq + Hash, B: Eq + Hash> {
    map: HashMap<(A, B), f64>,
}

impl<A: Eq + Hash, B: Eq + Hash> Grid<A, B> {
    pub fn new() -> Self {
        Grid {
            map: HashMap::new(),
        }
    }

    pub fn get(&self, a: &A, b: &B) -> f64 {
        *self.map.get(&(a, b) as &dyn Coord<A, B>).unwrap()
    }

    pub fn set(&mut self, a: A, b: B, v: f64) {
        self.map.insert((a, b), v);
    }
}

impl<A, B> Coord<A, B> for (A, B) {
    fn row(&self) -> &A {
        &self.0
    }
    fn col(&self) -> &B {
        &self.1
    }
}
impl<A, B> Coord<A, B> for (&A, &B) {
    fn row(&self) -> &A {
        self.0
    }
    fn col(&self) -> &B {
        self.1
    }
}

// mod tests {
//     use crate::{
//         cell::Cell,
//         grid::{Coord, Grid},
//     };

//     #[test]
//     fn insert_some_cells() {
//         let mut grid = Grid::new(1);

//         let cell = grid.insert(
//             Coord {
//                 arr: 0,
//                 row: 0,
//                 col: 0,
//             },
//             Cell::Empty,
//         );
//         assert!(cell.is_some());

//         let cell = grid.get(&Coord {
//             arr: 0,
//             row: 0,
//             col: 0,
//         });

//         assert!(cell.is_some());
//     }
// }
