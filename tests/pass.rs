#![allow(unused)]

use cell_project::cell_project as project;

use std::cell::Cell;

mod a {
    #[derive(Debug, PartialEq)]
    pub struct Foo {
        pub name: String,
        pub build: i32,
    }

    #[derive(Debug, PartialEq)]
    pub struct Bar<T> {
        pub name: String,
        pub build: T,
    }

    #[derive(Debug, PartialEq)]
    pub struct Quax<T, U> {
        pub name: String,
        pub build: T,
        pub value: U,
    }
}

#[test]
fn project_simple() {
    let foo = Cell::new(a::Foo {
        name: String::new(),
        build: 32,
    });

    project!(a::Foo, foo.name).set("String".to_string());

    assert_eq!(
        Cell::into_inner(foo),
        a::Foo {
            name: "String".to_string(),
            build: 32
        }
    )
}

#[test]
fn double_project() {
    let bar = Cell::new(a::Bar {
        name: String::new(),
        build: a::Foo {
            name: String::new(),
            build: 0,
        },
    });

    let build = project!(a::Bar<_>, bar.build);
    let build2 = project!(a::Foo, build.build);
    build2.set(32);

    assert_eq!(
        Cell::into_inner(bar),
        a::Bar {
            name: String::new(),
            build: a::Foo {
                name: String::new(),
                build: 32
            }
        }
    )
}

#[test]
fn aliasing() {
    let bar = Cell::new(a::Bar {
        name: String::new(),
        build: 31,
    });

    let build = project!(a::Bar<_>, bar.build);

    assert_eq!(build.get(), 31);

    bar.set(a::Bar {
        name: "hello".to_string(),
        build: 0,
    });

    assert_eq!(build.get(), 0);

    assert_eq!(
        Cell::into_inner(bar),
        a::Bar {
            name: "hello".to_string(),
            build: 0
        }
    )
}
