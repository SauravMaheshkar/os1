//! Multiboot header and structures.
//!  
//! References:
//! * <https://wiki.osdev.org/Multiboot>
//! * <https://www.gnu.org/software/grub/manual/multiboot/multiboot.html>
//! * <https://git.savannah.gnu.org/cgit/grub.git/tree/doc/multiboot.h?h=multiboot>

#[repr(C, packed)]
pub struct MulitbootInfo {
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

impl MulitbootInfo {
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
            println!("* Memory: {length}K, Address: {addr:#X}",);
        }

        println!("Total memory: {}M", total_memory as f32 / 1024.0 / 1024.0);
    }
}

#[repr(C, packed)]
pub struct MultibootMmapEntry {
    size: u32,
    addr: u64,
    length: u64,
    mmap_type: u32,
}
