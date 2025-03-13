#![no_std]
#![feature(abi_x86_interrupt)]

use bootloader_api::BootInfo;
use x86_64::{instructions, VirtAddr};

pub mod devices;
pub mod drivers;
pub mod graphics;
pub mod interrupts;
pub mod logger;
pub mod mm;
pub mod task;

/// Initializes the kernel by setting up the logger, GDT, IDT, enabling
/// interrupts, initializing the heap, and initializing the drivers.
///
/// # Arguments
/// * `framework_info` - The [`BootInfo`] struct that contains the information
///  passed from the bootloader to the kernel. This must be a static mutable
/// reference.
/// * `frame_buffer_logger_status` - A boolean that determines if the frame
///   buffer
/// logger should be enabled.
/// * `serial_logger_status` - A boolean that determines if the serial logger
/// should be enabled.
pub fn init(
    framework_info: &'static mut BootInfo,
    frame_buffer_logger_status: bool,
    serial_logger_status: bool,
) {
    // initialize logger
    let framebuffer = framework_info.framebuffer.take().unwrap();
    let info = framebuffer.info();
    let buffer = framebuffer.into_buffer();
    logger::init(
        info,
        buffer,
        frame_buffer_logger_status,
        serial_logger_status,
    );

    // initialize GDT, IDT, and enable interrupts
    interrupts::gdt::init();
    interrupts::idt::init();
    x86_64::instructions::interrupts::enable();

    // initialize heap and memory allocator
    let physical_memory_offset = VirtAddr::new(
        framework_info
            .physical_memory_offset
            .take()
            .expect("Failed to find physical memory offset"),
    );
    let mut mapper = mm::paging::init(physical_memory_offset);
    let mut allocator =
        mm::paging::BootInfoFrameAllocator::new(&framework_info.memory_regions);
    mm::allocator::init_heap(&mut mapper, &mut allocator)
        .expect("heap initialization failed");

    // initialize drivers
    let rsdp_addr = framework_info
        .rsdp_addr
        .take()
        .expect("Failed to find RSDP address");
    unsafe {
        drivers::init(
            rsdp_addr as usize,
            physical_memory_offset,
            &mut mapper,
            &mut allocator,
        );
    }
}

/// Halts the CPU by triggering the [`x86_64::instructions::hlt`] instruction in
/// a loop.
pub fn hlt_loop() -> ! {
    loop {
        instructions::hlt();
    }
}
