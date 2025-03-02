//! Interrupt Descriptor Table (IDT) implementation.
//!
//! # References
//! * https://wiki.osdev.org/Interrupt_Descriptor_Table
//! * https://wiki.osdev.org/8259_PIC

use core::{arch::asm, cell::UnsafeCell};

use crate::{io::PortHandler, util::bit_manipulation::BitManipulation};

struct GateDescriptorParams {
    offset: u32, // address of the entry point of the Interrupt Service Routine
    selector: u16, // Segment Selector pointing to a valid code segment in GDT
    gate_type: u8, /* Five types of gates:
                 0x5: Task Gate (offset unused and set to 0)
                 0x4: 16-bit Interrupt Gate
                 0x7: 16-bit Trap Gate
                 0xE: 32-bit Interrupt Gate
                 0xF: 32-bit Trap Gate
                 */
    dpl: u8, // Descriptor Privilege Level
    p: bool, // Present bit, must be 1 for all valid IDT entries
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
struct GateDescriptor(u64);

impl GateDescriptor {
    fn new(params: GateDescriptorParams) -> GateDescriptor {
        let mut gate_descriptor = 0u64;

        // Set the offset
        gate_descriptor.set_bits(0, 16, params.offset as u64);
        gate_descriptor.set_bits(48, 16, (params.offset >> 16) as u64);
        // Set the selector
        gate_descriptor.set_bits(16, 16, params.selector as u64);
        // Set the gate type
        gate_descriptor.set_bits(40, 4, params.gate_type as u64);
        // Set the DPL
        gate_descriptor.set_bits(45, 2, params.dpl as u64);
        // Set the present bit
        gate_descriptor.set_bit(47, params.p);

        GateDescriptor(gate_descriptor)
    }
}

struct InterruptTable {
    table: UnsafeCell<[GateDescriptor; 256]>,
}

impl InterruptTable {
    const fn new() -> Self {
        Self {
            table: UnsafeCell::new([GateDescriptor(0); 256]),
        }
    }
}

unsafe impl Sync for InterruptTable {}

static INTERRUPT_TABLE: InterruptTable = InterruptTable::new();

#[repr(C, packed)]
#[derive(Debug)]
pub struct IDT {
    size: u16,
    offset: u32,
}

pub fn sidt() -> IDT {
    let mut ret = core::mem::MaybeUninit::uninit();

    unsafe {
        asm!("sidt ({})", in(reg) ret.as_mut_ptr(), options(nostack, preserves_flags, att_syntax));

        ret.assume_init()
    }
}

pub fn init(port_handler: &mut PortHandler) {
    // Setup ports
    let mut pic1_data = port_handler
        .add(0x21)
        .expect("Failed to add port 0x21 (pic 1 data)");
    let mut pic2_data = port_handler
        .add(0xA1)
        .expect("Failed to add port 0xA1 (pic 2 data)");

    // disable ext interrupts
    pic1_data.writeb(0xff);
    pic2_data.writeb(0xff);

    let general_protection_fault_descriptor =
        GateDescriptor::new(GateDescriptorParams {
            offset: crate::interrupts::general_protection_fault as u32,
            selector: 0x08,
            gate_type: 0b1111,
            dpl: 0,
            p: true,
        });

    unsafe {
        // Construct the IDT
        let interrupt_table = &mut *INTERRUPT_TABLE.table.get();
        interrupt_table[13] = general_protection_fault_descriptor;
        let interrupt_table_ptr: *const GateDescriptor =
            interrupt_table.as_ptr();

        let idt = IDT {
            size: 256 * 8 - 1,
            offset: interrupt_table_ptr as u32,
        };

        // Load the IDT
        asm!(
            r#"
            lidt ({idt})
            sti
            int $13
            "#,
            idt = in(reg) &idt,
            options(att_syntax)
        );
    }

    println_serial!("{:?}", sidt());
}
