use alloc::alloc::{GlobalAlloc, Layout};
use core::ptr;

/// A wrapper around spin::Mutex to permit trait implementations.
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

/// Align the given address `addr` upwards to alignment `align`.
///
/// Requires that `align` is a power of two.
fn align_up(addr: usize, align: usize) -> usize {
    (addr + align - 1) & !(align - 1)
}

pub struct BumpAllocator {
    heap_start: usize,
    heap_end: usize,
    next: usize,
    allocations: usize,
}

impl BumpAllocator {
    /// Creates a new empty bump allocator.
    pub const fn new() -> Self {
        BumpAllocator {
            heap_start: 0,
            heap_end: 0,
            next: 0,
            allocations: 0,
        }
    }

    /// Initializes the bump allocator with the given heap bounds.
    ///
    /// This method is unsafe because the caller must ensure that the given
    /// memory range is unused. Also, this method must be called only once.
    pub unsafe fn init(&mut self, heap_start: usize, heap_size: usize) {
        self.heap_start = heap_start;
        self.heap_end = heap_start.saturating_add(heap_size);
        self.next = heap_start;
    }
}

unsafe impl GlobalAlloc for Locked<BumpAllocator> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut bump = self.lock(); // get a mutable reference

        let alloc_start = align_up(bump.next, layout.align());
        let alloc_end = match alloc_start.checked_add(layout.size()) {
            Some(end) => end,
            None => return ptr::null_mut(),
        };

        if alloc_end > bump.heap_end {
            ptr::null_mut() // out of memory
        } else {
            bump.next = alloc_end;
            bump.allocations += 1;
            alloc_start as *mut u8
        }
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        let mut bump = self.lock(); // get a mutable reference

        bump.allocations -= 1;
        if bump.allocations == 0 {
            bump.next = bump.heap_start;
        }
    }
}

#[global_allocator]
static ALLOCATOR: Locked<BumpAllocator> = Locked::new(BumpAllocator::new());

#[cfg(test)]
mod test {
    use core::sync::atomic::Ordering;

    use crate::mem::{
        // allocator::ALLOCATOR,
        segment::{AllocatedSegment, MemorySegment},
    };

    fn get_state() -> [MemorySegment; 42] {
        unsafe {
            let mut state = [MemorySegment {
                size: 0,
                next: core::ptr::null_mut(),
            }; 42];
            let mut count = 0;
            // let mut iter = ALLOCATOR.head.load(Ordering::Relaxed);

            // while !iter.is_null() {
            //     state[count] = *iter;
            //     count += 1;
            //     iter = (*iter).next;
            // }

            state
        }
    }

    #[test_case]
    pub fn test_alloc_state_changes() {
        print_serial!(
            "[TEST] Assert memory state changes after allocation ... "
        );
        use alloc::boxed::Box;

        let initial_state = get_state();
        let fill = Box::new(4);
        let new_state = get_state();
        assert_ne!(initial_state, new_state);

        let alloc_state = get_state();
        let num_diff = initial_state
            .iter()
            .zip(alloc_state.iter())
            .filter(|(a, b)| a != b)
            .count();
        assert_eq!(num_diff, 1);

        let diff_item = initial_state
            .iter()
            .zip(alloc_state.iter())
            .find(|(a, b)| a != b)
            .expect("could not find a != b");

        let before = core::ptr::addr_of!(diff_item.0.size);
        let after = core::ptr::addr_of!(diff_item.1.size);
        // We can only test that at least the given memory has been allocated
        // because we do not know the state of alignment before the
        // allocation
        unsafe {
            assert!(
                before.read_unaligned()
                    > after.read_unaligned()
                        + 4
                        + core::mem::size_of::<AllocatedSegment>()
            );
        }

        drop(fill);

        println_serial!("✓");
    }
}
