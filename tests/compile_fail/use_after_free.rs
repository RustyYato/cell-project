use cell_project::cell_project as project;
use std::cell::Cell;

pub struct Foo(u32);

fn main() {
    let foo = Cell::new(Foo(0));
    let field = project!(Foo, foo.0);
    drop(foo);
    field.get();
}
