//! # Memory Management
//!
//! * [`allocator`] - Custom Implementation of a Allocator
//! * [`paging`] - Implementations of Paging

pub mod allocator;
pub mod paging;

// A wrapper around spin::Mutex to permit trait implementations.
pub struct Locked<A> {
    inner: spin::Mutex<A>,
}

impl<A> Locked<A> {
    pub const fn new(inner: A) -> Self {
        Locked {
            inner: spin::Mutex::new(inner),
        }
    }

    pub fn lock(&self) -> spin::MutexGuard<A> {
        self.inner.lock()
    }
}
