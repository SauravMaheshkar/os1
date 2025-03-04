//! Interrupt Descriptor Table (IDT) implementation.
//!
//! # References
//! * https://wiki.osdev.org/Interrupt_Descriptor_Table
//! * https://wiki.osdev.org/8259_PIC

use lazy_static::lazy_static;
use x86_64::structures::idt::InterruptDescriptorTable;

use crate::interrupts::breakpoint_handler;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        // idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt
    };
}

pub fn init_idt() {
    // IDT.load();
}
