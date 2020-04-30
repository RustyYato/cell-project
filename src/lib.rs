#![no_std]

#[doc(hidden)]
pub use core::{cell::Cell, compile_error, concat, stringify};

#[doc(hidden)]
#[allow(unused)]
#[inline(always)]
pub unsafe fn project_unchecked<T, F>(cell: &Cell<T>, field: *const F, offset: usize) -> &Cell<F> {
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

#[macro_export]
macro_rules! cell_project {
    ($type:path, $ident:ident.$field:ident) => {
        match $ident {
            ref cell => unsafe {
                let cell: &$crate::Cell<$type> = cell;
                let ptr = cell.as_ptr();
                let $type { $field: field, .. } = &*ptr;
                let field = field as *const _;
                let offset = (field as *const () as usize).wrapping_sub(ptr as *mut () as usize);
                $crate::project_unchecked(cell, field, offset)
            },
        }
    };
}

#[macro_export]
#[cfg(feature = "nightly")]
macro_rules! nightly_cell_project {
    ($type:path, $ident:ident.$field:ident) => {
        match $ident {
            ref cell => unsafe {
                let cell: &$crate::Cell<$type> = cell;
                let ptr = cell.as_ptr();
                $crate::nightly_project_unchecked(cell, &raw const (*ptr).$field, 0)
            },
        }
    };
}
