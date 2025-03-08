use x86_64::{
    structures::paging::{FrameAllocator, Mapper, Size4KiB},
    VirtAddr,
};

pub mod acpi;
pub mod apic;

extern crate acpi as acpi_lib;

pub fn disable_pic() {
    use x86_64::instructions::port::Port;

    unsafe {
        Port::<u8>::new(0xA1).write(0xFF); // PIC2 (Slave PIC)
    }
}

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
