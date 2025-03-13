//! Memory Management module.
pub mod allocator;
pub mod paging;

/// A simple wrapper around spin::Mutex to provide a locked value.
pub struct Locked<A> {
    inner: spin::Mutex<A>,
}

impl<A> Locked<A> {
    /// Create a new Locked instance.
    pub const fn new(inner: A) -> Self {
        Locked {
            inner: spin::Mutex::new(inner),
        }
    }

    /// Lock the value and return a MutexGuard.
    pub fn lock(&self) -> spin::MutexGuard<A> {
        self.inner.lock()
    }
}
