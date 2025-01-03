use core::mem::{self, MaybeUninit};

// based on:
// - https://github.com/rust-lang/rust/blob/1.83.0/library/core/src/mem/maybe_uninit.rs

pub(crate) fn copy_from_slice<'a, T>(this: &'a mut [MaybeUninit<T>], src: &[T]) -> &'a mut [T]
where
    T: Copy,
{
    // SAFETY: &[T] and &[MaybeUninit<T>] have the same layout
    let uninit_src: &[MaybeUninit<T>] = unsafe { mem::transmute(src) };

    this.copy_from_slice(uninit_src);

    // SAFETY: Valid elements have just been copied into `this` so it is initialized
    // unsafe { MaybeUninit::slice_assume_init_mut(this) }
    unsafe { &mut *(this as *mut [MaybeUninit<T>] as *mut [T]) }
    // unsafe { &mut *(this as *mut [MaybeUninit<T>]) }
}
