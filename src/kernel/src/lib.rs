#![no_std]
#![feature(abi_x86_interrupt)]

use bootloader_api::BootInfo;
use x86_64::VirtAddr;

pub mod devices;
pub mod drivers;
pub mod graphics;
pub mod interrupts;
pub mod logger;
pub mod mm;
pub mod task;

pub fn init(
    framework_info: &'static mut BootInfo,
    frame_buffer_logger_status: bool,
    serial_logger_status: bool,
) {
    let framebuffer = framework_info.framebuffer.take().unwrap();
    let info = framebuffer.info();
    let buffer = framebuffer.into_buffer();

    logger::init(
        info,
        buffer,
        frame_buffer_logger_status,
        serial_logger_status,
    );

    interrupts::gdt::init();
    interrupts::idt::init();

    x86_64::instructions::interrupts::enable();

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

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
