enum Cell {
    Empty(Coord),
    Post(Coord, usize),
    Fence(Coord),
    Loop(Coord),
}

enum Color {
    Green,
    Yellow,
    Orange,
    Red,
    Violet,
    Blue,
    White,
}

struct Coord {
    x: i32,
    y: i32,
    z: i32,
    color: Color,
}

struct Grid {
    tiles: Vec<Cell>,
}

impl Grid {
    fn new(width: usize, posts: Vec<Cell>) -> Self {
        todo!()
    }
}

fn main() {
    println!("Hello, world!");
}

mod tests {
    use crate::{Cell, Color, Coord};

    #[test]
    fn nominal_case() {
        let posts = vec![
            Cell::Post(
                Coord {
                    x: 0,
                    y: 0,
                    z: 0,
                    color: Color::White,
                },
                4,
            ),
            Cell::Post(
                Coord {
                    x: 0,
                    y: 3,
                    z: -3,
                    color: Color::Blue,
                },
                4,
            ),
            Cell::Post(
                Coord {
                    x: -3,
                    y: 2,
                    z: 1,
                    color: Color::Violet,
                },
                4,
            ),
            Cell::Post(
                Coord {
                    x: 3,
                    y: -2,
                    z: -1,
                    color: Color::Yellow,
                },
                5,
            ),
            Cell::Post(
                Coord {
                    x: 0,
                    y: -3,
                    z: 3,
                    color: Color::Orange,
                },
                2,
            ),
        ];

        assert!(false);
    }
}
