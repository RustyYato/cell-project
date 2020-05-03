pub use core::{cell::Cell, mem, ptr};

#[inline(always)]
pub unsafe fn project_unchecked<T: ?Sized, F: ?Sized>(cell: &Cell<T>, field: *const F) -> &Cell<F> {
    let mut ptr = cell.as_ptr();

    assert_eq!(
        core::alloc::Layout::new::<*mut T>(),
        core::alloc::Layout::new::<*mut F>()
    );

    // This dance of two copies is to maintain the providence of `Cell::as_ptr`
    // this first copy shouldn't change the providence of `ptr`, but it set's all
    // of it's bits to that of `field`
    (&mut ptr as *mut _ as *mut *const F).copy_from_nonoverlapping(&field, 1);

    // the second copy should have the same providence as `ptr`, while pointing to field
    let ptr: *mut F = mem::transmute_copy(&ptr);
    &*(ptr as *mut Cell<F>)
}

#[inline(always)]
#[cfg(feature = "nightly")]
pub unsafe fn nightly_project_unchecked<T: ?Sized, F: ?Sized>(
    _: &Cell<T>,
    field: *const F,
) -> &Cell<F> {
    &*(field as *const Cell<F>)
}
