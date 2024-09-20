use std::cell::UnsafeCell;

/// Temporary until the std's version is stabilized.
#[derive(Default)]
#[repr(transparent)]
pub(crate) struct SyncUnsafeCell<T: ?Sized> {
    value: UnsafeCell<T>,
}

unsafe impl<T: Sync + ?Sized> Sync for SyncUnsafeCell<T> {}

impl<T> SyncUnsafeCell<T> {
    pub const fn new(value: T) -> Self {
        Self { value: UnsafeCell::new(value) }
    }

    pub const fn get(&self) -> *mut T {
        self.value.get()
    }
}
