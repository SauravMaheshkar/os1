/// Struct for a segment of memory.
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MemorySegment {
    pub size: usize,
    pub next: *mut MemorySegment,
}

impl MemorySegment {
    /// Get a pointer to the head
    pub unsafe fn get_head(&self) -> *mut u8 {
        (self as *const MemorySegment).add(1) as *mut u8
    }

    /// Get a pointer to the tail
    pub unsafe fn get_tail(&self) -> *mut u8 {
        self.get_head().add(self.size)
    }

    /// Set tail
    pub unsafe fn set_tail(&mut self, tail: *mut u8) {
        self.size = tail
            .offset_from(self.get_tail())
            .try_into()
            .expect("End should be > than start");
    }
}

/// Struct for an allocated segment of memory.
#[allow(dead_code)]
pub struct AllocatedSegment {
    /// Size of the allocated segment.
    pub size: usize,

    /// Padding to align the size of the allocated segment.
    pub padding: [u8; 4],
}

/// Get the head of an [`AllocatedSegment`] given a pointer to it.
///
/// # Arguments
/// * `ptr` - mutable `u8` pointer to the segment
///
/// # Returns
/// * mutable pointer (type [`AllocatedSegment`]) to the segment head
pub unsafe fn get_head_of_allocated_segment(ptr: *mut u8) -> *mut AllocatedSegment {
    let head = ptr.sub(core::mem::size_of::<AllocatedSegment>());
    head as *mut AllocatedSegment
}

/// Deallocates a given segment of memory
///
/// # Arguments
/// * `head` - pointer to a free segment of a memory. Check usage in allocator.
/// * `head_ptr` - pointer to a allocated segment.
pub unsafe fn deallocate_segment(head: *mut MemorySegment, head_ptr: *mut AllocatedSegment) {
    let segment_size = (*head_ptr).size;
    let segment = head_ptr as *mut MemorySegment;

    (*segment).size = segment_size;
    (*segment).next = core::ptr::null_mut();

    let mut iter = head;
    while !iter.is_null() {
        assert!(
            iter < segment,
            "trying to deallocate more memory than already allocated"
        );

        if (*iter).next.is_null() || (*iter).next > segment {
            let next = (*iter).next;
            (*iter).next = segment;
            (*segment).next = next;

            merge_segments(segment, (*segment).next);
            merge_segments(iter, segment);
            return;
        }

        iter = (*iter).next;
    }

    panic!("Failed to deallocate segment!");
}

unsafe fn merge_segments(segment_a: *mut MemorySegment, segment_b: *mut MemorySegment) {
    if (*segment_a).get_tail() == segment_b as *mut u8 {
        (*segment_a).set_tail((*segment_b).get_tail());
        (*segment_a).next = (*segment_b).next;
    }
}
