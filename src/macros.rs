pub use core::{cell::Cell, mem, ptr};

unsafe fn validate_fat_pointer_layout<T: ?Sized>(ptr: &mut *mut T) -> *mut *mut u8 {
    let addr = *ptr as *mut () as usize;
    let ptr_ptr = ptr as *mut *mut T as *mut *mut u8;

    if core::mem::size_of::<*mut T>() != core::mem::size_of::<*mut ()>() {
        assert_eq!(
            core::mem::size_of::<*mut T>(),
            2 * core::mem::size_of::<*mut ()>(),
            "cell-project cannot handle custom fat pointer layouts"
        );

        // assert that the data pointer is the in the first word
        assert_eq!(
            addr, *ptr_ptr as usize,
            "cell-project cannot handle custom fat pointer layouts"
        );

        // assert that the data pointer is not the in the second word
        assert_ne!(
            addr,
            *ptr_ptr.add(1) as usize,
            "cell-project cannot handle custom fat pointer layouts"
        );
    }

    ptr_ptr
}

#[inline(always)]
pub unsafe fn project_unchecked<T: ?Sized, F>(
    cell: &Cell<T>,
    field: *const F,
) -> &Cell<F> {
    let mut ptr = cell.as_ptr();

    assert_eq!(core::alloc::Layout::new::<*mut T>(), core::alloc::Layout::new::<*mut F>());

    validate_fat_pointer_layout(&mut ptr);
    validate_fat_pointer_layout(&mut (field as *mut F));

    (&mut ptr as *mut _ as *mut *const F).copy_from_nonoverlapping(&field, 1);
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
