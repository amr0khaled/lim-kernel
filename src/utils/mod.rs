pub mod asm;

use core::cell::UnsafeCell;

pub struct SyncT<T> {
    inner: UnsafeCell<T>,
}

#[allow(dead_code)]
impl<T> SyncT<T> {
    pub const fn new(inner: T) -> Self {
        Self {
            inner: UnsafeCell::new(inner),
        }
    }
    pub const fn get(&self) -> &mut T {
        unsafe { &mut (*self.inner.get()) }
    }
    pub const fn raw(&self) -> *mut T {
        self.inner.get()
    }
}

unsafe impl<T> Sync for SyncT<T> {}
