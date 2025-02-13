use core::alloc::{GlobalAlloc, Layout};
use core::sync::atomic::AtomicPtr;
use core::sync::atomic::Ordering::Relaxed;

use crate::multiboot::MultibootInfo;
use crate::{KERNEL_END_ADDR, KERNEL_START_ADDR};

#[repr(C, packed)]
struct MemorySegment {
    size: usize,
    next: *mut MemorySegment,
}

impl MemorySegment {
    unsafe fn get_head(&self) -> *mut u8 {
        (self as *const MemorySegment).add(1) as *mut u8
    }

    unsafe fn get_tail(&self) -> *mut u8 {
        self.get_head().add(self.size)
    }
}

#[allow(dead_code)]
struct AllocatedSegment {
    size: usize,
    padding: [u8; 4],
}

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

        println!("Reserved memory: {:#x}", reserved_memory);

        // Calculate the block size
        let block_size =
            block.length as usize - reserved_memory - core::mem::size_of::<MemorySegment>();

        println!("Block size: {:?}", block_size as f32 / 1024.0);

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

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        panic!("dealloc not implemented");
    }
}
