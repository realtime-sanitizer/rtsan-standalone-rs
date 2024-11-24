use std::sync::{LockResult, MutexGuard, TryLockResult};

use rtsan_macros::blocking;

pub struct Mutex<T: ?Sized> {
    inner: std::sync::Mutex<T>,
}

unsafe impl<T: ?Sized + Send> Send for Mutex<T> {}
unsafe impl<T: ?Sized + Send> Sync for Mutex<T> {}

impl<T> Mutex<T> {
    pub fn new(t: T) -> Self {
        Self {
            inner: std::sync::Mutex::new(t),
        }
    }

    #[blocking]
    pub fn lock(&self) -> LockResult<MutexGuard<T>> {
        self.inner.lock()
    }

    pub fn try_lock(&self) -> TryLockResult<MutexGuard<T>> {
        self.inner.try_lock()
    }

    pub fn is_poisoned(&self) -> bool {
        self.inner.is_poisoned()
    }

    pub fn clear_poison(&self) {
        self.inner.clear_poison();
    }

    pub fn get_mut(&mut self) -> Result<&mut T, std::sync::PoisonError<&mut T>> {
        self.inner.get_mut()
    }

    pub fn into_innter(self) -> LockResult<T> {
        self.inner.into_inner()
    }
}
