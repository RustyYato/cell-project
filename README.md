# cell-project

A safe interface to project through shared references to [`core::cell::Cell`](https://!doc.rust-lang.org/core/cell/struct.Cell.html).

Documentation:
https://docs.rs/cell-project

```rust
use std::cell::Cell;
use cell_project::cell_project as cp; // renamed for ergonomics

struct Point {
    x: f32,
    y: f32,
}

fn get_x_cell(point: &Cell<Point>) -> &Cell<f32> {
    cp!(Point, point.x)
}
```

The syntax for the macro is as follows

```rust compile_fail
let projection = cp!($TypeOfValue, $value_identifier.$field_identifier);
```

You may not pass an expression for `$value_identifier`, if you need to then you should do.

```rust
let value = Cell::new(get_point());
let projection = cp!(Point, value.y);
```

If you need to project through multiple fields then you need to call `cp!` multiple times, once per projection

```rust
struct Pair<T>(T, T);

// let some_pair: &Cell<Pair<Point>>;
let point = cp!(Pair<Point>, some_pair.0);
let x = cp!(Point, point.x);
```

note: for generic types, you can use `_` to infer the generic parameters

```rust
fn get_x_cell<T>(point: &Cell<Pair<T>>) -> &Cell<T> {
    cp!(Pair<_>, point.0)
}
```

Some limitations, you cannot project an enum variant because that is potentially unsound.

```rust compile_fail
## cell_project::docs_example!{}
let x = Cell::new(Some(0));

// let's imagine a macro like `try_cell_project`, which takes a varaint as well as a type
let proj = cell_project::try_cell_project!(Option<_>, Some, x.0).unwrap();

x.set(None); // we can still write to the `Cell` directly

// this will read uninitialized memory (because that's what `None` wrote in)
// and there is no way to fix this. Enums cannot allow safe projection through
// a shared mutable reference (like `&Cell<_>`)
let _ = proj.get();
```rust
so you cannot project through enums

Another limitation of stable, you can only project to `Sized` types. For example, if I have a type

```rust
struct Unsized(i32, [u8]);
```
Then I can only project to the first field, because the second field is `!Sized`

### features

`nightly` - unlocks `cell_project::nightly_cell_project`, which uses the unstable `#![feature(raw_ref_op)]` to
allow projections to `!Sized` fields.

License: MIT
