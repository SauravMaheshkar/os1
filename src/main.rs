#![allow(bad_asm_style)]
#![no_std]
#![no_main]

#[macro_use]
mod macros;

mod multiboot;
mod tui;

use multiboot::MulitbootInfo;
use tui::TerminalWriter;

use core::arch::global_asm;
use core::panic::PanicInfo;

global_asm!(include_str!("boot.s"));

#[no_mangle]
/// # Safety
///
/// This function should not be called before the horsemen are ready.
pub unsafe extern "C" fn kernel_main(
    mulitboot_magic: u32,
    _multiboot_info: *const MulitbootInfo,
) -> i32 {
    TerminalWriter::init();

    // Multiboot(1)-compliant bootloaders report themselves
    // with magic number 0x2BADB002
    assert_eq!(mulitboot_magic, 0x2BADB002);

    // Print bootloader name
    let boot_loader_name = (*_multiboot_info).boot_loader_name;
    println_str!("Using bootloader: {}", boot_loader_name);

    unsafe {
        multiboot::describe_mmap_sections(_multiboot_info);
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
