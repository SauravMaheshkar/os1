//! Multiboot header and structures.
//!
//! References:
//! * <https://wiki.osdev.org/Multiboot>
//! * <https://www.gnu.org/software/grub/manual/multiboot/multiboot.html>
//! * <https://git.savannah.gnu.org/cgit/grub.git/tree/doc/multiboot.h?h=multiboot>

use crate::{KERNEL_END_ADDR, KERNEL_START_ADDR};

#[repr(C, packed)]
pub struct MultibootInfo {
    flags: u32,
    mem_lower: u32,
    mem_upper: u32,
    boot_device: u32,
    cmdline: u32,
    mods_count: u32,
    mods_addr: u32,
    syms: [u8; 16],
    pub mmap_length: u32,
    pub mmap_addr: u32,
    drives_length: u32,
    drives_addr: u32,
    config_table: u32,
    pub boot_loader_name: *const u8,
    apm_table: u32,
}

impl MultibootInfo {
    pub unsafe fn get_name(&self) -> &str {
        core::ffi::CStr::from_ptr(self.boot_loader_name as *const i8)
            .to_str()
            .expect("Invalid UTF-8 string")
    }

    pub unsafe fn get_mmap_entries(&self) -> &[MultibootMmapEntry] {
        // Number of entries in the memory map
        let num_entries = (self.mmap_length as usize)
            / core::mem::size_of::<MultibootMmapEntry>();

        // Return a slice of the memory map entries
        core::slice::from_raw_parts(
            self.mmap_addr as *const MultibootMmapEntry,
            num_entries,
        )
    }

    pub unsafe fn describe(&self) {
        println_vga!("Kernel start: {:?}", &KERNEL_START_ADDR as *const u32);
        println_vga!("Kernel end: {:?}", &KERNEL_END_ADDR as *const u32);

        let _entries = unsafe { self.get_mmap_entries() };
        let mut total_memory = 0;
        for i in 0.._entries.len() as u32 {
            // Calculate the offset of the memory map entry
            let offset = core::mem::size_of::<MultibootMmapEntry>() as u32 * i;
            let mmap_entry = self
                .mmap_addr
                .checked_add(offset)
                .expect("memory map entry address overflow")
                as *const MultibootMmapEntry;

            // Print the memory map entry
            let length = (*mmap_entry).length as f32 / 1024.0;
            let addr = (*mmap_entry).addr;
            total_memory += (*mmap_entry).length;
            println_vga!("* Memory: {length}K, Address: {addr:#X}",);
        }

        println_vga!(
            "Total memory: {}M",
            total_memory as f32 / 1024.0 / 1024.0
        );
    }
}

#[repr(C, packed)]
pub struct MultibootMmapEntry {
    pub size: u32,
    pub addr: u64,
    pub length: u64,
    pub mmap_type: u32,
}
