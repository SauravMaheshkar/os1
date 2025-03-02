use core::{
    arch::asm,
    cell::UnsafeCell,
    ops::{Deref, DerefMut},
    sync::atomic::{AtomicUsize, Ordering},
};

static NUM_GUARDS: AtomicUsize = AtomicUsize::new(0);

pub struct InterruptGuard<'a, T> {
    data: &'a mut T,
}

impl<'a, T> Drop for InterruptGuard<'a, T> {
    fn drop(&mut self) {
        if NUM_GUARDS.fetch_sub(1, Ordering::SeqCst) == 1 {
            unsafe {
                asm!("sti");
            }
        }
    }
}

impl<'a, T> Deref for InterruptGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.data
    }
}

impl<'a, T> DerefMut for InterruptGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        self.data
    }
}

pub struct InterruptLock<T> {
    data: UnsafeCell<T>,
}

impl<T> InterruptLock<T> {
    pub const fn new(data: T) -> InterruptLock<T> {
        InterruptLock {
            data: UnsafeCell::new(data),
        }
    }

    pub fn lock(&self) -> InterruptGuard<'_, T> {
        NUM_GUARDS.fetch_add(1, Ordering::SeqCst);

        unsafe {
            asm!("cli");
        }

        unsafe {
            InterruptGuard {
                data: &mut *self.data.get(),
            }
        }
    }
}

// NOTE: Sync implementation assumes single threaded os
unsafe impl<T> Sync for InterruptLock<T> {}
