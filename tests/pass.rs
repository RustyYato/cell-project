#![cfg_attr(feature = "nightly", feature(raw_ref_op))]
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
    pub struct Bar<T: ?Sized> {
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

    build.set(10);

    assert_eq!(
        Cell::into_inner(bar),
        a::Bar {
            name: "hello".to_string(),
            build: 10
        }
    )
}

#[test]
fn slice_field() {
    let bar = Cell::new(a::Bar {
        name: String::new(),
        build: [31, 12, 13, 41],
    });

    {
        let bar: &Cell<a::Bar<[_]>> = &bar;
        let build: &Cell<[_]> = project!(a::Bar<_>, bar.build);

        for item in build.as_slice_of_cells() {
            item.set(0);
        }
    }

    assert_eq!(
        Cell::into_inner(bar),
        a::Bar {
            name: String::new(),
            build: [0; 4],
        }
    );
}

#[test]
fn trait_field() {
    let bar = Cell::new(a::Bar {
        name: String::new(),
        build: [31, 12, 13, 41],
    });

    {
        let bar: &Cell<a::Bar<[_]>> = &bar;
        let build: &Cell<[_]> = project!(a::Bar<_>, bar.build);

        for item in build.as_slice_of_cells() {
            item.set(0);
        }
    }

    assert_eq!(
        Cell::into_inner(bar),
        a::Bar {
            name: String::new(),
            build: [0; 4],
        }
    );
}

#[test]
#[cfg(feature = "nightly")]
fn nightly_unsized_field() {
    let bar = Cell::new(a::Bar {
        name: String::new(),
        build: [31, 12, 13, 41],
    });

    {
        let bar: &Cell<a::Bar<[_]>> = &bar;
        let build: &Cell<[_]> = cell_project::nightly_cell_project!(a::Bar<_>, bar.build);

        for item in build.as_slice_of_cells() {
            item.set(0);
        }
    }

    assert_eq!(
        Cell::into_inner(bar),
        a::Bar {
            name: String::new(),
            build: [0; 4],
        }
    );
}
