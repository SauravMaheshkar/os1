#![no_std]
#![feature(abi_x86_interrupt)]

use bootloader_api::BootInfo;

pub mod interrupts;
pub mod logger;

pub fn init(
    framework_info: &'static mut BootInfo,
    frame_buffer_logger_status: bool,
    serial_logger_status: bool,
) {
    logger::init(
        framework_info,
        frame_buffer_logger_status,
        serial_logger_status,
    );

    interrupts::gdt::init();
    interrupts::idt::init();

    x86_64::instructions::interrupts::enable();
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
