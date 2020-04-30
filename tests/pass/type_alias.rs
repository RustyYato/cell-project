use cell_project::cell_project as project;
use std::cell::Cell;

pub struct Foo {
    bar: i32,
    quax: i32,
}

pub struct Quax;

type Bar<T> = <T as Barry>::Output;
pub trait Barry {
    type Output;
}

impl Barry for Quax {
    type Output = Foo;
}

fn project(cell: &Cell<Foo>) -> &Cell<i32> {
    project!(Bar<Quax>, cell.bar)
}

fn main() {}
