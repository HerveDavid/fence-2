use std::rc::Rc;

#[derive(Debug, Clone, Copy)]
enum Cell {
    Empty(),
    Post,
}

enum Region {
    White,
    Green(Rc<(i32, i32, i32)>, Rc<(i32, i32, i32)>),
    Yellow(Rc<(i32, i32, i32)>, Rc<(i32, i32, i32)>),
    Orange(Rc<(i32, i32, i32)>, Rc<(i32, i32, i32)>),
    Red(Rc<(i32, i32, i32)>, Rc<(i32, i32, i32)>),
    Violet(Rc<(i32, i32, i32)>, Rc<(i32, i32, i32)>),
    Blue(Rc<(i32, i32, i32)>, Rc<(i32, i32, i32)>),
}
