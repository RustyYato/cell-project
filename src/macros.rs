pub use core::{cell::Cell, mem, ptr};

#[inline(always)]
pub unsafe fn project_unchecked<T: ?Sized, F>(
    cell: &Cell<T>,
    _: *mut F,
    offset: usize,
) -> &Cell<F> {
    &*cell.as_ptr().cast::<u8>().add(offset).cast::<Cell<F>>()
}

#[inline(always)]
#[cfg(feature = "nightly")]
pub unsafe fn nightly_project_unchecked<T: ?Sized, F: ?Sized>(
    _: &Cell<T>,
    field: *mut F,
) -> &Cell<F> {
    &*(field as *const Cell<F>)
}
