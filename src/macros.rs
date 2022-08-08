pub use core::{cell::Cell, mem, ptr};

#[inline(always)]
pub unsafe fn project_unchecked<T: ?Sized, F: ?Sized>(_: &Cell<T>, field_ptr: *mut F) -> &Cell<F> {
    &*(field_ptr as *mut Cell<F>)
}
