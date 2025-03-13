//! Paging module
use bootloader_api::info::{MemoryRegionKind::Usable, MemoryRegions};
use x86_64::{
    registers::control::Cr3,
    structures::paging::{
        FrameAllocator, OffsetPageTable, PageTable, PhysFrame, Size4KiB,
    },
    PhysAddr, VirtAddr,
};

/// A FrameAllocator that returns usable frames from the bootloader's memory
/// map.
pub struct BootInfoFrameAllocator {
    /// The memory regions from the bootloader.
    memory_map: &'static MemoryRegions,
    /// The next frame to return.
    next: usize,
}

impl BootInfoFrameAllocator {
    /// Create a new BootInfoFrameAllocator.
    pub fn new(memory_map: &'static MemoryRegions) -> Self {
        BootInfoFrameAllocator {
            memory_map,
            next: 0,
        }
    }

    /// Returns an iterator over the usable frames from the memory map.
    pub fn usable_frames(&self) -> impl Iterator<Item = PhysFrame> {
        let regions = self.memory_map.iter();

        let usable_regions = regions.filter(|region| region.kind == Usable);
        let address_ranges =
            usable_regions.map(|region| region.start..region.end);
        let frame_addresses =
            address_ranges.flat_map(|region| region.step_by(4096));

        frame_addresses.map(|address| {
            PhysFrame::containing_address(PhysAddr::new(address))
        })
    }
}

unsafe impl FrameAllocator<Size4KiB> for BootInfoFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame<Size4KiB>> {
        let frame = self.usable_frames().nth(self.next);
        self.next += 1;
        frame
    }
}

/// Initialize the offset page table.
///
/// # Arguments
/// * `physical_memory_offset` - The offset of the physical memory.
///
/// # Returns
/// A static offset page table.
pub fn init(physical_memory_offset: VirtAddr) -> OffsetPageTable<'static> {
    let level4_table = active_level4_table(physical_memory_offset);
    unsafe { OffsetPageTable::new(level4_table, physical_memory_offset) }
}

/// Get a mutable ptr to the level 4 table.
///
/// # Arguments
/// * `physical_memory_offset` - The offset of the physical memory.
///
/// # Returns
/// A mutable ptr to the level 4 table.
fn active_level4_table(
    physical_memory_offset: VirtAddr,
) -> &'static mut PageTable {
    let (level4_table_frame, _) = Cr3::read();

    let physical_address = level4_table_frame.start_address();
    let virtual_address = physical_memory_offset + physical_address.as_u64();
    let page_table_pointer: *mut PageTable = virtual_address.as_mut_ptr();

    unsafe { &mut *page_table_pointer }
}
