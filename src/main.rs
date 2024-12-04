#![no_std]
#![no_main]
#![allow(bad_asm_style)]

pub mod tui;

use core::arch::global_asm;
use core::panic::PanicInfo;

use tui::TerminalWriter;

global_asm!(include_str!("boot.s"));

/// The entry point of the kernel
///
/// This function is called by the boot code in `boot.s`
#[no_mangle]
pub extern "C" fn kernel_main() {
    let mut writer = TerminalWriter::new();
    writer.write(b"Hello, World!\n");
}

/// Simple panic handler that loops forever
///
/// # Arguments
/// * `_info` - The panic information
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
