use cell_project::cell_project as project;
use std::cell::Cell;

pub struct Foo {
    bar: i32,
    quax: i32,
}

fn project(cell: &Cell<Foo>) -> &Cell<i32> {
    project!(Foo, cell.foo)
}

fn main() {}
