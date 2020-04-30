#![no_std]

#[doc(hidden)]
pub use core::{cell::Cell, compile_error, concat, stringify};

#[doc(hidden)]
#[inline(always)]
pub unsafe fn project_unchecked<T, F>(cell: &Cell<T>, _: *const F, offset: usize) -> &Cell<F> {
    &*cell.as_ptr().cast::<u8>().add(offset).cast::<Cell<F>>()
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
