//! APIC (Advanced Programmable Interrupt Controller) driver
use x86_64::{
    structures::paging::{FrameAllocator, Mapper, PhysFrame, Size4KiB},
    PhysAddr, VirtAddr,
};

pub mod io_apic;
pub mod local_apic;
pub mod registers;

/// Map the APIC to the virtual address space
///
/// # Arguments
/// * `physical_address` - The physical address of the APIC
/// * `mapper` - The mapper to use for mapping
///   ([`x86_64::structures::paging::Mapper`])
/// * `frame_allocator` - The frame allocator to use for allocating frames
///  ([`x86_64::structures::paging::FrameAllocator`])
///
/// # Returns
/// start address of the mapped page
fn map_apic(
    physical_address: u64,
    mapper: &mut impl Mapper<Size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) -> VirtAddr {
    use x86_64::structures::paging::{Page, PageTableFlags as Flags};

    let physical_address = PhysAddr::new(physical_address);
    let page =
        Page::containing_address(VirtAddr::new(physical_address.as_u64()));
    let frame = PhysFrame::containing_address(physical_address);

    let flags = Flags::PRESENT | Flags::WRITABLE | Flags::NO_CACHE;

    unsafe {
        mapper
            .map_to(page, frame, flags, frame_allocator)
            .expect("APIC mapping failed")
            .flush();
    }

    page.start_address()
}

/// trigger end of interrupt by writing to the EOI Local APIC register
pub fn end_interrupt() {
    unsafe {
        let lapic_ptr = local_apic::LAPIC_ADDR.lock().address;
        lapic_ptr
            .offset(registers::APICRegisters::Eoi as isize / 4)
            .write_volatile(0);
    }
}
