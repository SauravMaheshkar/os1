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

#[repr(C, packed)]
pub struct MultibootMmapEntry {
    size: u32,
    pub addr: u64,
    pub length: u64,
    mmap_type: u32,
}
