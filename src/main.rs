#![allow(bad_asm_style)]
#![no_std]
#![no_main]

#[macro_use]
mod macros;

mod io;
mod mem;
mod multiboot;

use io::serial::Serial;
use io::vga::TerminalWriter;
use mem::allocator::Allocator;
use multiboot::MultibootInfo;

extern crate alloc;

use core::arch::global_asm;
use core::panic::PanicInfo;

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
    ALLOC.init(&*multiboot_info);

    TerminalWriter::init();
    Serial::init().expect("Error while initialising Serial Communication");

    // Canvas
    let vec = alloc::vec![1, 2, 3, 4, 5];
    println_vga!("vec: {:?}", vec);

    // Multiboot(1)-compliant bootloaders report themselves
    // with magic number 0x2BADB002
    assert_eq!(mulitboot_magic, 0x2BADB002);

    // Print bootloader name
    let boot_loader_name = (*multiboot_info).get_name();
    println_serial!("Using bootloader: {}", boot_loader_name);

    unsafe {
        (*multiboot_info).describe();
    }
    0
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // Print the message passed to `panic!`
    // Source:
    // * https://doc.rust-lang.org/beta/core/panic/struct.PanicInfo.html#method.message
    println_vga!("Panic: {}", _info.message());
    loop {}
}
