use core::{
    cell::UnsafeCell,
    ops::{Deref, DerefMut},
    sync::atomic::AtomicBool,
};

pub struct Guard<'a, T> {
    data: &'a mut T,
    free: &'a AtomicBool,
}

impl<'a, T> Drop for Guard<'a, T> {
    fn drop(&mut self) {
        self.free.store(true, core::sync::atomic::Ordering::SeqCst);
    }
}

impl<'a, T> Deref for Guard<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.data
    }
}

impl<'a, T> DerefMut for Guard<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        self.data
    }
}

#[allow(dead_code)]
pub struct Lock<T> {
    data: UnsafeCell<T>,
    free: AtomicBool,
}

#[allow(dead_code)]
impl<T> Lock<T> {
    pub const fn new(data: T) -> Lock<T> {
        Self {
            data: UnsafeCell::new(data),
            free: AtomicBool::new(true),
        }
    }

    pub fn lock(&self) -> Guard<'_, T> {
        while self
            .free
            .compare_exchange_weak(
                true,
                false,
                core::sync::atomic::Ordering::AcqRel,
                core::sync::atomic::Ordering::Relaxed,
            )
            .is_err()
        {}

        assert!(!self.free.load(core::sync::atomic::Ordering::Acquire));

        unsafe {
            Guard {
                data: &mut *self.data.get(),
                free: &self.free,
            }
        }
    }
}

unsafe impl<T> Sync for Lock<T> {}
unsafe impl<T: Send> Send for Lock<T> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_case]
    fn test_lock() {
        print_serial!("[TEST] Assert that a lock locks ... ");

        let lock = Lock::new(0);

        let mut guard = lock.lock();
        *guard += 1;
        assert_eq!(*guard, 1);

        println_serial!("✓");
    }

    #[test_case]
    fn test_lock_drop() {
        print_serial!("[TEST] Assert that a lock unlocks ... ");

        let lock = Lock::new(0);

        {
            let mut guard = lock.lock();
            *guard += 1;
            assert_eq!(*guard, 1);
        }

        let mut guard = lock.lock();
        *guard += 1;
        assert_eq!(*guard, 2);

        println_serial!("✓");
    }
}
