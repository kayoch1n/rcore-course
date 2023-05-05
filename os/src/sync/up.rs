use core::cell::{RefCell, RefMut};

pub struct UPSafeCell<T> {
    #[allow(dead_code)]
    /// inner
    inner: RefCell<T>,
}

unsafe impl<T> Sync for UPSafeCell<T> {}

impl<T> UPSafeCell<T> {
    #[allow(dead_code)]
    pub unsafe fn new(value: T) -> Self {
        Self {
            inner: RefCell::new(value),
        }
    }
    #[allow(dead_code)]
    pub fn exclusive_access(&self) -> RefMut<'_, T> {
        self.inner.borrow_mut()
    }
}
