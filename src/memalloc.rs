use core::alloc::{GlobalAlloc, Layout};

pub struct Allocator;

impl Allocator {
    pub const fn new() -> Self {
        Self
    }
}

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        panic!("alloc not implemented");
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        panic!("dealloc not implemented");
    }
}
