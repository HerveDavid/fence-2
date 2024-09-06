use grid::Grid;

mod cell;
mod grid;

#[derive(Eq, PartialEq, Hash)]
struct A(&'static str);

#[derive(Eq, PartialEq, Hash)]
struct B(&'static str);

fn main() {
    let mut table = Grid::new();
    table.set(A("abc"), B("def"), 4.0);
    table.set(A("123"), B("456"), 45.0);
    println!("{:?} == 45.0?", table.get(&A("123"), &B("456")));
    println!("{:?} == 4.0?", table.get(&A("abc"), &B("def")));
    // Should panic below.
    println!("{:?} == NaN?", table.get(&A("123"), &B("def")));
}
