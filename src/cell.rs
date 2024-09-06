#[derive(Debug, Clone, Copy)]
pub enum Cell {
    Empty(Region),
    Post,
}

#[derive(Debug, Clone, Copy)]
pub enum Region {
    White,
    Green,
    Yellow,
    Orange,
    Red,
    Violet,
    Blue,
}
