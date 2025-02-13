#![allow(bad_asm_style)]
#![no_std]
#![no_main]

#[macro_use]
mod macros;

mod memalloc;
mod multiboot;
mod tui;

use multiboot::MultibootInfo;
use tui::TerminalWriter;

extern crate alloc;

use core::arch::global_asm;
use core::panic::PanicInfo;

#[global_allocator]
static ALLOC: memalloc::Allocator = memalloc::Allocator::new();

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
    TerminalWriter::init();
    ALLOC.init(&*multiboot_info);

    // Canvas
    let vec = alloc::vec![1];
    println!("vec: {:?}", vec);

    // Multiboot(1)-compliant bootloaders report themselves
    // with magic number 0x2BADB002
    assert_eq!(mulitboot_magic, 0x2BADB002);

    // Print bootloader name
    let boot_loader_name = (*multiboot_info).boot_loader_name;
    println_str!("Using bootloader: {}", boot_loader_name);

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
    println!("Panic: {}", _info.message());
    loop {}
}
