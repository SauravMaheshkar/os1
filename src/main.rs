#![allow(bad_asm_style)]
#![no_std]
#![no_main]

mod multiboot;
mod tui;

use core::arch::global_asm;
use core::panic::PanicInfo;

use multiboot::{MulitbootInfo, MultibootMmapEntry};
use tui::TerminalWriter;

global_asm!(include_str!("boot.s"));

#[no_mangle]
/// # Safety
///
/// This function should not be called before the horsemen are ready.
pub unsafe extern "C" fn kernel_main(mulitboot_magic: u32, info: *const MulitbootInfo) -> i32 {
    let mut writer = TerminalWriter::new();

    // Multiboot(1)-compliant bootloaders report themselves
    // with magic number 0x2BADB002
    assert_eq!(mulitboot_magic, 0x2BADB002);

    // Print bootloader name
    let boot_loader_name: *const u8 = (*info).boot_loader_name;
    writer.write(b"Bootloader: ");
    writer.write(core::slice::from_raw_parts(boot_loader_name, 4));
    writer.write(b"\n");

    for i in 0..(*info).mmap_length {
        let offset = core::mem::size_of::<MultibootMmapEntry>() as u32 * i;
        let mmap_entry = (*info)
            .mmap_addr
            .checked_add(offset)
            .expect("memory map entry address overflow")
            as *const MultibootMmapEntry;

        writer.write(b"Length: ");
        writer.put_u32((*mmap_entry).length as u32);
        writer.write(b" Address: ");
        writer.put_u32((*mmap_entry).addr as u32);
        writer.write(b"\n");
    }
    0
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
