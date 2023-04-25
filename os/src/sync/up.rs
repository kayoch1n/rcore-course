use core::cell::{RefCell, RefMut};

use crate::debug;

pub struct UPSafeCell<T> {
    /// inner
    inner: RefCell<T>,
}

unsafe impl<T> Sync for UPSafeCell<T> {}

impl<T> UPSafeCell<T> {
    pub unsafe fn new(value: T) -> Self {
        Self {
            inner: RefCell::new(value),
        }
    }

    pub fn exclusive_access(&self) -> RefMut<'_, T> {
        debug!("borrow happens");
        self.inner.borrow_mut()
    }
}
