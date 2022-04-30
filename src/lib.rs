#![no_std]

//! A safe interface to project through shared references to [`core::cell::Cell`](https://!doc.rust-lang.org/core/cell/struct.Cell.html).
//!
//! ```rust
//! use std::cell::Cell;
//! use cell_project::cell_project as cp; // renamed for ergonomics
//!
//! struct Point {
//!     x: f32,
//!     y: f32,
//! }
//!
//! fn get_x_cell(point: &Cell<Point>) -> &Cell<f32> {
//!     cp!(Point, point.x)
//! }
//! ```
//!
//! The syntax for the macro is as follows
//!
//! ```rust compile_fail
//! let projection = cp!($TypeOfValue, $value_identifier.$field_identifier);
//! ```
//!
//! You may not pass an expression for `$value_identifier`, if you need to then you should do.
//!
//! ```rust
//! # cell_project::docs_example!{point}
//! # fn get_point() -> Point { Point { x: 0.0, y: 0.0 } }
//! let value = Cell::new(get_point());
//! let projection = cp!(Point, value.y);
//! ```
//!
//! If you need to project through multiple fields then you need to call `cp!` multiple times, once per projection
//!
//! ```rust
//! # cell_project::docs_example!{point}
//! struct Pair<T>(T, T);
//!
//! # fn some_point(some_pair: &Cell<Pair<Point>>) {
//! // let some_pair: &Cell<Pair<Point>>;
//! let point = cp!(Pair<Point>, some_pair.0);
//! let x = cp!(Point, point.x);
//! # }
//! ```
//!
//! note: for generic types, you can use `_` to infer the generic parameters
//!
//! ```rust
//! # cell_project::docs_example!{pair}
//! fn get_x_cell<T>(point: &Cell<Pair<T>>) -> &Cell<T> {
//!     cp!(Pair<_>, point.0)
//! }
//! ```
//!
//! Some limitations, you cannot project an enum variant because that is potentially unsound.
//!
//! ```rust compile_fail
//! # cell_project::docs_example!{}
//! let x = Cell::new(Some(0));
//!
//! // let's imagine a macro like `try_cell_project`, which takes a varaint as well as a type
//! let proj = cell_project::try_cell_project!(Option<_>, Some, x.0).unwrap();
//!
//! x.set(None); // we can still write to the `Cell` directly
//!
//! // this will read uninitialized memory (because that's what `None` wrote in)
//! // and there is no way to fix this. Enums cannot allow safe projection through
//! // a shared mutable reference (like `&Cell<_>`)
//! let _ = proj.get();
//! ```
//! so you cannot project through enums
//!
//! Another limitation of stable, you can only project to `Sized` types. For example, if I have a type
//!
//! ```rust
//! struct Unsized(i32, [u8]);
//! ```
//! Then I can only project to the first field, because the second field is `!Sized`
//!
//! ## features
//!
//! `nightly` - unlocks [`cell_project::nightly_cell_project`](nightly_cell_project), which uses the unstable `#![feature(raw_ref_op)]` to
//! allow projections to `!Sized` fields.

#[doc(hidden)]
pub mod macros;

#[doc(hidden)]
#[macro_export]
macro_rules! docs_example {
    () => {
        use cell_project::cell_project as cp; // renamed for ergonomics
        use std::cell::Cell;
    };
    (point) => {
        use cell_project::cell_project as cp;
        use std::cell::Cell; // renamed for ergonomics

        struct Point {
            x: f32,
            y: f32,
        }
    };
    (pair) => {
        $crate::docs_example! {point}
        pub struct Pair<T>(T, T);
    };
}

/// project through a shared mutable reference `&Cell`
///
/// see the crate docs for more information
#[macro_export]
macro_rules! cell_project {
    ($type:path, $ident:ident.$field:tt) => {
        match $ident {
            ref cell => unsafe {
                let cell: &$crate::macros::Cell<$type> = cell;
                let ptr = cell.as_ptr();
                let $type { $field: field, .. } = &mut *ptr;
                let field = field as *mut _;
                let offset = (field as *const () as usize) - (ptr as *const () as usize);
                $crate::macros::project_unchecked(cell, field, offset)
            },
        }
    };
}

/// project through a shared mutable reference `&Cell`
///
/// see the crate docs for more information
#[macro_export]
#[cfg(any(feature = "nightly", doc))]
macro_rules! nightly_cell_project {
    ($type:path, $ident:ident.$field:tt) => {
        match $ident {
            ref cell => unsafe {
                let cell: &$crate::macros::Cell<$type> = cell;
                let ptr = cell.as_ptr();
                let $type { $field: field, .. } = &*ptr;
                $crate::macros::nightly_project_unchecked(cell, &raw mut (*ptr).$field)
            },
        }
    };
}
