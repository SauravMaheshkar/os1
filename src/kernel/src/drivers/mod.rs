//! ACPI + APIC drivers
use x86_64::{
    structures::paging::{FrameAllocator, Mapper, Size4KiB},
    VirtAddr,
};

pub mod acpi;
pub mod apic;

extern crate acpi as acpi_lib;

/// Disable the legacy PIC (Programmable Interrupt Controller)
pub fn disable_pic() {
    use x86_64::instructions::port::Port;

    unsafe {
        Port::<u8>::new(0xA1).write(0xFF); // PIC2 (Slave PIC)
    }
}

/// Map the APIC to the virtual address space
///
/// initialize the ACPI + APIC (both local and I/O) and disable the legacy PIC
///
/// # Arguments
/// * `rsdp` - The physical address of the RSDP
/// * `physical_memory_offset` - The physical memory offset as a
///   [`x86_64::VirtAddr`]
/// * `mapper` - The mapper to use for mapping
///   ([`x86_64::structures::paging::Mapper`])
/// * `frame_allocator` - The frame allocator to use for allocating frames
///  ([`x86_64::structures::paging::FrameAllocator`])
pub unsafe fn init(
    rsdp: usize,
    physical_memory_offset: VirtAddr,
    mapper: &mut impl Mapper<Size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) {
    let acpi = acpi::ACPI::new(physical_memory_offset);

    let acpi_tables = acpi_lib::AcpiTables::from_rsdp(acpi, rsdp)
        .expect("Failed to parse ACPI tables");

    let platform_info = acpi_tables
        .platform_info()
        .expect("Failed to get platform info");

    match platform_info.interrupt_model {
        acpi_lib::InterruptModel::Apic(apic) => {
            let io_apic_addr = apic.io_apics[0].address;
            apic::io_apic::init(io_apic_addr as usize, mapper, frame_allocator);

            let local_apic_addr = apic.local_apic_address;
            apic::local_apic::init_local_apic(
                local_apic_addr as usize,
                mapper,
                frame_allocator,
            );
        }
        _ => {
            panic!("Unsupported interrupt model");
        }
    }

    disable_pic();
}
