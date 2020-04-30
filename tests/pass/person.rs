use cell_project::cell_project as project;
use std::cell::Cell;

struct Person {
    name: String,
    age: usize,
}

pub trait PersonExt {
    fn age_nmut(&self) -> &Cell<usize>;
}

impl PersonExt for Cell<Person> {
    fn age_nmut(&self) -> &Cell<usize> {
        project!(Person, self.age)
    }
}

fn replace_first_with_last(party: &mut [Person]) {
    let party = Cell::from_mut(party).as_slice_of_cells();
    if let (Some(f), Some(l)) = (party.first(), party.last()) {
        f.age_nmut().set(l.age_nmut().get());
    }
}

fn main() {}
