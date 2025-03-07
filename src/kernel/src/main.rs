#![no_std]
#![no_main]

use core::panic::PanicInfo;

use bootloader_api::{
    config::{BootloaderConfig, Mapping},
    entry_point, BootInfo,
};

pub static BOOTLOADER_CONFIG: BootloaderConfig = {
    let mut config = BootloaderConfig::new_default();
    config.mappings.physical_memory = Some(Mapping::Dynamic);
    config
};

entry_point!(kernel_main, config = &BOOTLOADER_CONFIG);

/// The entry point of the kernel
///
/// This function is called by the boot code in `boot.s`
#[no_mangle]
fn kernel_main(info: &'static mut BootInfo) -> ! {
    kernel::init(info, true, true);
    kernel::hlt_loop();
}

/// Simple panic handler that loops forever
///
/// # Arguments
/// * `_info` - The panic information
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    log::info!("[PANIC]: {}", info);
    kernel::hlt_loop();
}
