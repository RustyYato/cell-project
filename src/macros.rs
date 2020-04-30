#[doc(hidden)]
pub use core::{cell::Cell, compile_error, concat, stringify};

#[doc(hidden)]
#[allow(unused)]
#[inline(always)]
pub unsafe fn project_unchecked<T: ?Sized, F>(
    cell: &Cell<T>,
    field: *const F,
    offset: usize,
) -> &Cell<F> {
    &*cell.as_ptr().cast::<u8>().add(offset).cast::<Cell<F>>()
}

#[doc(hidden)]
#[allow(unused)]
#[inline(always)]
#[cfg(feature = "nightly")]
pub unsafe fn nightly_project_unchecked<T: ?Sized, F: ?Sized>(
    cell: &Cell<T>,
    field: *const F,
    offset: usize,
) -> &Cell<F> {
    &*(field as *const Cell<F>)
}
