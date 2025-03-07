#![no_std]

use bootloader_api::BootInfo;

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
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
