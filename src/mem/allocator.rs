use core::alloc::{GlobalAlloc, Layout};
use core::sync::atomic::AtomicPtr;
use core::sync::atomic::Ordering::Relaxed;

use crate::mem::segment::{
    deallocate_segment, get_head_of_allocated_segment, AllocatedSegment, MemorySegment,
};
use crate::multiboot::MultibootInfo;
use crate::{KERNEL_END_ADDR, KERNEL_START_ADDR};

pub struct Allocator {
    head: AtomicPtr<MemorySegment>,
}

impl Allocator {
    pub const fn new() -> Self {
        Self {
            head: AtomicPtr::new(core::ptr::null_mut()),
        }
    }

    pub unsafe fn init(&self, info: &MultibootInfo) {
        assert_eq!(
            core::mem::size_of::<MemorySegment>(),
            core::mem::size_of::<AllocatedSegment>()
        );

        // Find the memory block containing the kernel
        let block = info
            .get_mmap_entries()
            .iter()
            .find(|entry| entry.addr == &KERNEL_START_ADDR as *const u32 as u64)
            .expect("failed to find kernel start address");

        // Calculate the reserved memory
        let reserved_memory =
            (&KERNEL_END_ADDR as *const u32 as usize) - (&KERNEL_START_ADDR as *const u32 as usize);

        println_vga!("Reserved memory: {:#x}", reserved_memory);

        // Calculate the block size
        let block_size =
            block.length as usize - reserved_memory - core::mem::size_of::<MemorySegment>();

        println_vga!("Block size: {:?}", block_size as f32 / 1024.0);

        // Determine the address of the segment
        let segment_addr = (&KERNEL_END_ADDR as *const u32 as usize) as *mut u8;

        // Align the segment address to the alignment of MemorySegment
        let alignment = core::mem::align_of::<MemorySegment>();
        let aligned_addr = (segment_addr as usize + alignment - 1) & !(alignment - 1);
        let aligned_segment_addr = aligned_addr as *mut MemorySegment;

        // Ensure the address is properly aligned for MemorySegment
        assert_eq!(
            aligned_segment_addr as usize % core::mem::align_of::<MemorySegment>(),
            0,
            "Memory segment address must be properly aligned"
        );

        // Initialize the memory segment
        *aligned_segment_addr = MemorySegment {
            size: block_size,
            next: core::ptr::null_mut(),
        };

        // Set the head of Allocator to the memory segment address
        self.head.store(aligned_segment_addr, Relaxed);
    }
}

unsafe fn get_metadata(memory_segment: &MemorySegment, layout: &Layout) -> Option<*mut u8> {
    let head = memory_segment.get_head();
    let tail = memory_segment.get_tail();

    let mut ptr = tail.sub(layout.size());
    ptr = ptr.sub((ptr as usize) % layout.align());
    ptr = ptr.sub(core::mem::size_of::<AllocatedSegment>());

    if ptr < head {
        return None;
    }

    Some(ptr)
}

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut block_iter = self.head.load(Relaxed);

        while !block_iter.is_null() {
            let metadata = get_metadata(&*block_iter, &layout);

            let metadata = match metadata {
                Some(metadata) => metadata,
                None => {
                    block_iter = (*block_iter).next;
                    continue;
                }
            };

            let tail = (*block_iter).get_tail();

            let size = metadata
                .offset_from((*block_iter).get_head())
                .try_into()
                .expect("Expected usize for segment");
            (*block_iter).size = size;

            let metadata = metadata as *mut AllocatedSegment;
            (*metadata).size = tail
                .offset_from(metadata.add(1) as *mut u8)
                .try_into()
                .expect("Expected usize for segment tail offset");

            return metadata.add(1) as *mut u8;
        }
        panic!("Failed to allocate memory");
    }

    /// Custom deallocation function which deallocates a block of memory
    /// at the given `ptr` address with the given `layout`.
    ///
    /// # Arguments
    /// * `ptr` - The pointer to the block of memory currently allocated
    /// * `layout` - The same layout as the one used to allocate the memory
    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        let head_ptr = get_head_of_allocated_segment(ptr);

        deallocate_segment(self.head.load(Relaxed), head_ptr);
    }
}

#[cfg(test)]
mod test {
    use core::sync::atomic::Ordering;

    use crate::{mem::segment::MemorySegment, ALLOC};

    fn get_state() -> [MemorySegment; 42] {
        unsafe {
            let mut state = [MemorySegment {
                size: 0,
                next: core::ptr::null_mut(),
            }; 42];
            let mut count = 0;
            let mut iter = ALLOC.head.load(Ordering::Relaxed);

            while !iter.is_null() {
                state[count] = *iter;
                count += 1;
                iter = (*iter).next;
            }

            state
        }
    }

    #[test_case]
    pub fn test_alloc_state_changes() {
        print_serial!("[TEST] Assert memory state changes after allocation ... ");
        use alloc::boxed::Box;

        let initial_state = get_state();
        let _fill = Box::new(2);
        let new_state = get_state();
        assert_ne!(initial_state, new_state);

        println_serial!("✓");
    }
}
