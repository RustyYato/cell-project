use cell_project::cell_project as cp;
use core::cell::Cell;

struct Foo<'a> {
    x: Option<&'a Cell<Foo<'a>>>,
}

impl<'a> Drop for Foo<'a> {
    fn drop(&mut self) {
        // `ourselves` is an &Cell<Self>.
        // NB: `Drop` is unsound.
        if let Some(ourselves) = self.x.as_ref() {
            // replace `self` (but this doesn't actually replace `self`)
            let is_x_none = ourselves.replace(Foo { x: None }).x.as_ref().is_none();
            // if we just moved out of `self`, and we had a `Some` originally,
            // how come this is a `None`?
            if is_x_none {
                println!("how did we get a None?");
            }
        }
    }
}

fn main() {
    let foo = Cell::new(Foo { x: None });
    let x = cp!(Foo<'_>, foo.x);
    x.set(Some(&foo));
}
