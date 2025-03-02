#![allow(bad_asm_style)]
#![no_std]
#![no_main]
#![feature(maybe_uninit_uninit_array)]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![test_runner(testing::test_runner)]
#![reexport_test_harness_main = "test_main"]

#[macro_use]
mod macros;
#[macro_use]
mod logger;
mod interrupts;
mod io;
mod mem;
mod multiboot;
mod util;

#[cfg(test)]
mod testing;

use mem::allocator::Allocator;
use multiboot::MultibootInfo;

extern crate alloc;

use core::{arch::global_asm, panic::PanicInfo};

#[global_allocator]
static ALLOC: Allocator = Allocator::new();

global_asm!(include_str!("boot.s"));

extern "C" {
    static KERNEL_START_ADDR: u32;
    static KERNEL_END_ADDR: u32;
}

#[no_mangle]
/// # Safety
///
/// This function should not be called before the horsemen are ready.
pub unsafe extern "C" fn kernel_main(
    mulitboot_magic: u32,
    multiboot_info: *const MultibootInfo,
) -> i32 {
    let interrupt_guard = interrupts::guard::InterruptLock::new(());
    let interrupt_guard = interrupt_guard.lock();

    ALLOC.init(&*multiboot_info);
    logger::init(Default::default());

    let mut port_handler = io::PortHandler::new();
    io::init_io_ports(&mut port_handler);
    io::init_io_exit(&mut port_handler);

    #[cfg(test)]
    {
        test_main();
        io::exit(0);
    }

    interrupts::gdt::init();
    interrupts::idt::init(&mut port_handler);
    drop(interrupt_guard);

    unsafe {
        core::arch::asm!("int $13");
    }

    let mut rtc =
        io::rtc::Rtc::new(&mut port_handler).expect("Failed to initialise RTC");
    println_serial!("RTC: {:?}", rtc.get());

    // Canvas
    println_serial!("vec: {:?}", alloc::vec![1, 2, 3, 4, 5]);

    // Multiboot(1)-compliant bootloaders report themselves
    // with magic number 0x2BADB002
    assert_eq!(mulitboot_magic, 0x2BADB002);

    // Print bootloader name
    let boot_loader_name = (*multiboot_info).get_name();
    info!("Using bootloader: {}", boot_loader_name);

    // Print memory map
    // unsafe {
    //     (*multiboot_info).describe();
    // }

    logger::service();
    io::exit(0);
    0
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // Print the message passed to `panic!`
    // Source:
    // * https://doc.rust-lang.org/beta/core/panic/struct.PanicInfo.html#method.message
    println_vga!("Panic: {}", _info.message());
    unsafe {
        io::exit(1);
    }
    loop {}
}
