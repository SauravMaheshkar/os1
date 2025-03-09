#![no_std]
#![no_main]

extern crate alloc;

use core::panic::PanicInfo;

use bootloader_api::{
    config::{BootloaderConfig, Mapping},
    entry_point, BootInfo,
};
use kernel::{
    devices::keyboard,
    task::{executor, Task},
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
    let mut framebuffer = unsafe { core::ptr::read(&info.framebuffer) };

    kernel::init(info, true, true);

    kernel::graphics::examples::tga::draw_tga(&mut framebuffer.take().unwrap());

    let mut executor = executor::Executor::new();
    executor.spawn(Task::new(example_task()));
    executor.spawn(Task::new(keyboard::print_keypresses()));
    executor.run();
}

/// Simple panic handler that loops forever
///
/// # Arguments
/// * `_info` - The panic information
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    log::error!("[PANIC]: {}", info);
    kernel::hlt_loop();
}

async fn async_number() -> u32 {
    42
}

async fn example_task() {
    let number = async_number().await;
    log::info!("async number: {}", number);
}
