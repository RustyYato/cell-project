use cell_project::cell_project as project;
use std::cell::Cell;

pub struct Foo {
    bar: Bar,
    quax: i32,
}

pub struct Bar {
    foo: i32,
}

impl std::ops::Deref for Foo {
    type Target = Bar;

    fn deref(&self) -> &Self::Target {
        &self.bar
    }
}

fn check_deref_coercion(foo: &Foo) -> &i32 {
    &foo.foo
}

fn project(cell: &Cell<Foo>) -> &Cell<i32> {
    project!(Foo, cell.foo)
}

fn main() {}
