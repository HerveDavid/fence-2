use std::borrow::Borrow;
use std::hash::{Hash, Hasher};

pub trait Coord {
    fn a(&self) -> &i32;
    fn r(&self) -> &i32;
    fn c(&self) -> &i32;
}

impl Coord for (i32, i32, i32) {
    fn a(&self) -> &i32 {
        &self.0
    }
    fn r(&self) -> &i32 {
        &self.1
    }
    fn c(&self) -> &i32 {
        &self.2
    }
}

impl Coord for &(i32, i32, i32) {
    fn a(&self) -> &i32 {
        &self.0
    }
    fn r(&self) -> &i32 {
        &self.1
    }
    fn c(&self) -> &i32 {
        &self.2
    }
}

impl<'a> Borrow<dyn Coord + 'a> for (i32, i32, i32) {
    fn borrow(&self) -> &(dyn Coord + 'a) {
        self
    }
}

impl<'a> Borrow<dyn Coord + 'a> for &(i32, i32, i32) {
    fn borrow(&self) -> &(dyn Coord + 'a) {
        *self
    }
}

impl Hash for dyn Coord + '_ {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.a().hash(state);
        self.r().hash(state);
        self.c().hash(state);
    }
}

impl PartialEq for dyn Coord + '_ {
    fn eq(&self, other: &Self) -> bool {
        self.a() == other.a() && self.r() == other.r() && self.c() == other.c()
    }
}

impl Eq for dyn Coord + '_ {}

pub trait Near<A, B: Coord> {
    fn near(&self, coord: &A) -> B;
}

impl<C: Coord> Near<C, (i32, i32, i32)> for (i32, i32, i32) {
    fn near(&self, coord: &C) -> (i32, i32, i32) {
        (self.0 + coord.a(), self.1 + coord.r(), self.2 + coord.c())
    }
}
