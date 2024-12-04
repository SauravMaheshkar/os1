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
    addr_low: u32,
    addr_high: u32,
    pub length_low: u32,
    length_high: u32,
    mmap_type: u32,
}

pub unsafe fn describe_mmap_sections(multibootinfo: *const MulitbootInfo) {
    let mmap_length = (*multibootinfo).mmap_length;
    println!("mmap length: {mmap_length}");
    println!("Memory Segments:- ");

    for i in 0..(*multibootinfo).mmap_length {
        let offset = core::mem::size_of::<MultibootMmapEntry>() as u32 * i;
        let mmap_entry = (*multibootinfo)
            .mmap_addr
            .checked_add(offset)
            .expect("memory map entry address overflow")
            as *const MultibootMmapEntry;

        let length = (*mmap_entry).length_low;
        let size = (*mmap_entry).size;
        if size == 0 {
            break;
        }
        let addr = (*mmap_entry).addr_low;
        println!("* Size: {size}, Length: {length}, Address: {addr}");
    }
}
