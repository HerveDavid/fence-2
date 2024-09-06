#[derive(Debug, PartialEq, Eq)]
pub enum Cell {
    Empty,
    Post,
}

impl Default for Cell {
    fn default() -> Self {
        Self::Empty
    }
}
