//! I/O APIC (Advanced Programmable Interrupt Controller) module
use x86_64::structures::paging::{FrameAllocator, Mapper, Size4KiB};

use super::map_apic;

/// Initialize the I/O APIC
///
/// # Arguments
/// * `local_apic_addr` - The physical address of the I/O APIC
/// * `mapper` - The mapper to use for mapping
///   ([`x86_64::structures::paging::Mapper`])
/// * `frame_allocator` - The frame allocator to use for allocating frames
///  ([`x86_64::structures::paging::FrameAllocator`])
pub unsafe fn init(
    ioapic_address: usize,
    mapper: &mut impl Mapper<Size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) {
    let virt_addr = map_apic(ioapic_address as u64, mapper, frame_allocator);

    let ioapic_pointer = virt_addr.as_mut_ptr::<u32>();

    ioapic_pointer.offset(0).write_volatile(0x12);
    ioapic_pointer.offset(4).write_volatile(
        crate::interrupts::InterruptIndex::Keyboard as u8 as u32,
    );
}
