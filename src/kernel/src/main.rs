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

    let mut executor = executor::Executor::new();

    executor.spawn(Task::new(keyboard::print_keypresses()));

    // executor.spawn(Task::new(async move {
    //     kernel::graphics::examples::tga::draw_tga(
    //         &mut framebuffer.take().unwrap(),
    //     )
    //     .await;
    // }));

    executor.spawn(Task::new(async move {
        kernel::graphics::examples::bounce::bouncing_ball(
            &mut framebuffer.take().unwrap(),
        )
        .await;
    }));

    // executor.spawn(Task::new(async move {
    //     kernel::graphics::examples::magic_word::magic_word(
    //         &mut framebuffer.take().unwrap(),
    //     )
    //     .await;
    // }));

    executor.run();
}

/// Simple panic handler that loops forever
///
/// # Arguments
/// * `info` - The panic information
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    log::error!("[PANIC]: {}", info);
    kernel::hlt_loop();
}
