use crate::sync::atomic::Atomic;
use core::cell::UnsafeCell;
use core::ops::{Deref, DerefMut};

/// A mutual exclusion primitive useful for protecting shared data.
pub struct Mutex<T: ?Sized> {
    lock: Atomic<bool>,
    data: UnsafeCell<T>,
}

unsafe impl<T: ?Sized + Send> Send for Mutex<T> {}
unsafe impl<T: ?Sized + Send> Sync for Mutex<T> {}

impl<T> Mutex<T> {
    /// Creates a new mutex in an unlocked state ready for use.
    ///
    /// # Examples
    /// ```
    /// use nds::sync::Mutex;
    ///
    /// let mutex = Mutex::new(0);
    /// ```
    #[inline]
    pub const fn new(data: T) -> Self {
        Self {
            lock: Atomic::new(false),
            data: UnsafeCell::new(data),
        }
    }
}

impl<T: ?Sized> Mutex<T> {
    /// Acquires a mutex, blocking the current thread until it is able to do so.
    pub fn lock(&self) -> MutexGuard<'_, T> {
        while self.try_lock().is_none() {
            crate::thread::yield_now();
        }

        MutexGuard::new(self)
    }

    /// Attempts to acquire this lock.
    #[must_use]
    pub fn try_lock(&self) -> Option<MutexGuard<'_, T>> {
        match self.lock.compare_exchange(false, true) {
            Ok(_) => Some(MutexGuard::new(self)),
            Err(_) => None,
        }
    }
}

impl<T> From<T> for Mutex<T> {
    fn from(data: T) -> Self {
        Self::new(data)
    }
}

impl<T: Default> Default for Mutex<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

/// An RAII implementation of a "scoped lock" of a mutex.\
/// When this structure is dropped (falls out of scope), the lock will be unlocked.
pub struct MutexGuard<'m, T: ?Sized + 'm> {
    mutex: &'m Mutex<T>,
}

impl<T: ?Sized> !Send for MutexGuard<'_, T> {}
unsafe impl<T: ?Sized + Sync> Sync for MutexGuard<'_, T> {}

impl<'m, T: ?Sized> MutexGuard<'m, T> {
    const fn new(mutex: &'m Mutex<T>) -> Self {
        Self { mutex }
    }
}

impl<T: ?Sized> Deref for MutexGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // SAFETY: The lock is held, giving us exclusive access to the data.
        unsafe { &*self.mutex.data.get() }
    }
}

impl<T: ?Sized> DerefMut for MutexGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: The lock is held, giving us exclusive access to the data.
        unsafe { &mut *self.mutex.data.get() }
    }
}

impl<T: ?Sized> Drop for MutexGuard<'_, T> {
    #[inline]
    fn drop(&mut self) {
        self.mutex.lock.store(false);
    }
}
