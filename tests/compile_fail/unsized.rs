use cell_project::cell_project as project;
use std::cell::Cell;

pub struct Foo {
    bar: i32,
    quax: [u8],
}

fn check_unsized_field(foo: &Foo) -> &[u8] {
    &foo.quax
}

fn project(cell: &Cell<Foo>) -> &Cell<[u8]> {
    project!(Foo, cell.quax)
}

fn main() {}
