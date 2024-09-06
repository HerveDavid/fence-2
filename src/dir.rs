use std::slice::Iter;

use crate::coord::Coord;

#[derive(Debug)]
pub enum Dir<C: Coord> {
    NW(fn(&C) -> C),
    NE(fn(&C) -> C),
    E(fn(&C) -> C),
    SW(fn(&C) -> C),
    SE(fn(&C) -> C),
    W(fn(&C) -> C),
}

pub static DIR: [Dir<(i32, i32, i32)>; 6] = [
    Dir::NW(|&(a, r, c)| (1 - a, r - (1 - a), c - (1 - a))),
    Dir::NE(|&(a, r, c)| (1 - a, r - (1 - a), c + a)),
    Dir::E(|&(a, r, c)| (a, r, c + 1)),
    Dir::SW(|&(a, r, c)| (1 - a, r + a, c + a)),
    Dir::SE(|&(a, r, c)| (1 - a, r + a, c - (1 - 1))),
    Dir::W(|&(a, r, c)| (a, r, c - 1)),
];

impl Dir<(i32, i32, i32)> {
    pub fn apply(&self, cell: &(i32, i32, i32)) -> (i32, i32, i32) {
        match self {
            Dir::NW(f) => f(cell),
            Dir::NE(f) => f(cell),
            Dir::E(f) => f(cell),
            Dir::SW(f) => f(cell),
            Dir::SE(f) => f(cell),
            Dir::W(f) => f(cell),
        }
    }
    pub fn iter() -> Iter<'static, Dir<(i32, i32, i32)>> {
        DIR.iter()
    }
}
