use cell_project::cell_project as project;
use std::cell::Cell;

mod a {
    pub struct Foo {
        pub name: String,
        pub build: i32,
    }

    pub struct Bar<T> {
        pub name: String,
        pub build: T,
    }

    pub struct Quax<T, U> {
        pub name: String,
        pub build: T,
        pub value: U,
    }
}

fn generic<T>(bar: &mut a::Bar<T>) -> &Cell<String> {
    let bar = Cell::from_mut(bar);
    project!(a::Bar<_>, bar.name)
}

fn super_generic<T, U>(quax: &mut a::Quax<T, a::Quax<a::Bar<a::Foo>, a::Bar<U>>>) -> &Cell<String> {
    let quax = Cell::from_mut(quax);
    project!(a::Quax<_, a::Quax<a::Bar<a::Foo>, _>>, quax.name)
}

fn main() {}
