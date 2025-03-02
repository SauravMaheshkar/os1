//! Global Descriptor Table (GDT) implementation.
//!
//! # References
//! * https://wiki.osdev.org/Global_Descriptor_Table

use core::arch::asm;

use crate::util::bit_manipulation::BitManipulation;

struct AccessByteParams {
    p: bool, // present bit, must be 1 for all valid segment
    dpl: u8, /* 2 bits: 0 = highest privilege (kernel), 3 = lowest privilege
              * (user applications) */
    s: bool, // descriptor type, 0 = system, 1 = code or data
    e: bool, // executable bit, 0 = data segment, 1 = code segment
    dc: bool, /* direction/conforming bit
              * data segment: 0 = grows up, 1 = grows down
              * code segment: 0 = only from dpl, 1 = conforming (equal or
              * lower privilege)
              */
    rw: bool, /* read/write bit
               * code segment: 0 = read access denied, 1 = read access
               * allowed data segment: 0 = write access (write never)
               * denied, 1 = write access allowed (read never)
               */
    a: bool, /* accessed bit, set to 0 by CPU, set to 1 by CPU when segment
              * is accessed */
}

struct AccessByte {
    access_byte: u8,
}

impl AccessByte {
    const fn new() -> AccessByte {
        AccessByte { access_byte: 0 }
    }

    const fn get(&self) -> u8 {
        self.access_byte
    }

    fn build(&self, access_byte_params: AccessByteParams) -> u8 {
        let mut access_byte = self.get();

        access_byte.set_bit(7, access_byte_params.p);
        access_byte.set_bits(5, 2, access_byte_params.dpl);
        access_byte.set_bit(4, access_byte_params.s);
        access_byte.set_bit(3, access_byte_params.e);
        access_byte.set_bit(2, access_byte_params.dc);
        access_byte.set_bit(1, access_byte_params.rw);
        access_byte.set_bit(0, access_byte_params.a);

        access_byte
    }
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
struct SegmentDescriptor(u64);

#[allow(dead_code)]
impl SegmentDescriptor {
    fn new(
        base: u32,
        limit: u32,
        access_byte: u8,
        flags: u8,
    ) -> SegmentDescriptor {
        let mut descriptor: u64 = 0u64;

        descriptor.set_bits(0, 16, limit as u64);
        descriptor.set_bits(48, 4, (limit >> 16) as u64);

        descriptor.set_bits(16, 24, base.into());
        descriptor.set_bits(56, 8, (base >> 24).into());

        descriptor.set_bits(40, 8, access_byte.into());
        descriptor.set_bits(52, 4, flags.into());

        SegmentDescriptor(descriptor)
    }

    fn get_base(&self) -> u32 {
        // Prevent unaligned access
        let data = self.0;
        let mut base = data.get_bits(16, 24);
        let upper = data.get_bits(56, 8);
        base |= upper << 24;
        base as u32
    }

    fn get_limit(&self) -> u32 {
        // Prevent unaligned access
        let data = self.0;
        let mut limit = data.get_bits(0, 16);
        let upper = data.get_bits(48, 4);
        limit |= upper << 16;
        limit as u32
    }

    fn get_access_byte(&self) -> u8 {
        // Prevent unaligned access
        let data = self.0;
        data.get_bits(40, 8) as u8
    }

    fn get_flags(&self) -> u8 {
        // Prevent unaligned access
        let data = self.0;

        data.get_bits(52, 4) as u8
    }
}

#[repr(C, packed)]
#[allow(clippy::upper_case_acronyms)]
pub struct GDT {
    limit: u16,
    base: u32,
}

fn get_gdt_entries() -> [SegmentDescriptor; 3] {
    let code_access_byte = AccessByte::new().build(AccessByteParams {
        p: true,
        dpl: 0,
        s: true,
        e: true,
        dc: false,
        rw: false,
        a: true,
    });
    let code_descriptor =
        SegmentDescriptor::new(0, 0xffff_ffff, code_access_byte, 0b1100);

    let data_access_byte = AccessByte::new().build(AccessByteParams {
        p: true,
        dpl: 0,
        s: true,
        e: false,
        dc: false,
        rw: true,
        a: true,
    });
    let data_descriptor =
        SegmentDescriptor::new(0, 0xffff_ffff, data_access_byte, 0b1100);

    [SegmentDescriptor(0), code_descriptor, data_descriptor]
}

#[allow(dead_code)]
pub fn sgdt() -> GDT {
    let mut ret = core::mem::MaybeUninit::uninit();

    unsafe {
        asm!("sgdt ({})", in(reg) ret.as_mut_ptr(), options(nostack, preserves_flags, att_syntax));

        ret.assume_init()
    }
}

#[allow(dead_code)]
pub unsafe fn print_gdt() {
    let gdt = sgdt();

    let limit = gdt.limit + 1;
    let base = gdt.base as *const SegmentDescriptor;

    println_serial!("GDT: ");
    println_serial!("base: {base:?}, limit: {limit:#x}");
    for i in 0..(limit / 8) {
        println_serial!("Segment {i}");
        let segment = *base.add(i.into());

        println_serial!(
            "base: {:#x}, limit: {:#x}, access: {:#x}, flags: {:#x}",
            segment.get_base(),
            segment.get_limit(),
            segment.get_access_byte(),
            segment.get_flags()
        );
    }
}

pub fn init() {
    let gdt_entries =
        get_gdt_entries().to_vec().leak() as &[SegmentDescriptor];

    let limit = core::mem::size_of_val(gdt_entries) - 1;

    let gdt = GDT {
        limit: limit as u16,
        base: gdt_entries.as_ptr() as u32,
    };

    unsafe {
        // Assert interrupts are disabled
        let cpu_flags: i32;
        asm!(r#"
            pushf
            pop {cpu_flags}
            push {cpu_flags}
            popf
            "#,
            cpu_flags = out(reg) cpu_flags
        );

        assert_eq!(
            (cpu_flags >> 9) & 0x1,
            0,
            "Caller is responsible for disable/enabling interrupts"
        );

        // Load the GDT
        asm!(r#"
            lgdt ({gdt})
            jmp $0x08, $1f

            1:
            mov $0x10, {reg}
            mov {reg}, %ds
            mov {reg}, %es
            mov {reg}, %fs
            mov {reg}, %gs
            mov {reg}, %ss
            "#,
            gdt = in(reg) &gdt,
            reg = out(reg) _,
            options(att_syntax)
        );
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test_case]
    pub fn test_get_base() {
        print_serial!(
            "[TEST] Assert base is correctly extracted from a descriptor ... "
        );

        const DESCRIPTOR: SegmentDescriptor =
            SegmentDescriptor(0x1200_0034_5678_DEAD);
        assert_eq!(DESCRIPTOR.get_base(), 0x12345678);

        println_serial!("✓");
    }

    #[test_case]
    pub fn test_get_limit() {
        print_serial!(
            "[TEST] Assert limit is correctly extracted from a descriptor ... "
        );

        const DESCRIPTOR: SegmentDescriptor =
            SegmentDescriptor(0x1200_0034_5678_DEAD);
        assert_eq!(DESCRIPTOR.get_limit(), 0xdead);

        println_serial!("✓");
    }

    #[test_case]
    pub fn test_access_byte() {
        print_serial!("[TEST] Assert access byte is correctly built ... ");

        let access_byte = AccessByte::new().build(AccessByteParams {
            p: true,
            dpl: 0,
            s: true,
            e: true,
            dc: false,
            rw: false,
            a: true,
        });
        assert_eq!(access_byte, 0b1001_1001);

        println_serial!("✓");
    }

    #[test_case]
    pub fn test_segment_descriptor() {
        print_serial!(
            "[TEST] Assert segment descriptor is correctly built ... "
        );

        const BASE: u32 = 0x12345678;
        const LIMIT: u32 = 0xdead;

        let descriptor = SegmentDescriptor::new(BASE, LIMIT, 0x65, 0x4);

        assert_eq!(descriptor.get_base(), BASE);
        assert_eq!(descriptor.get_limit(), LIMIT);
        assert_eq!(descriptor.get_access_byte(), 0x65);
        assert_eq!(descriptor.get_flags(), 0x4);

        println_serial!("✓");
    }
}
