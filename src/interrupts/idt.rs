//! Interrupt Descriptor Table (IDT) implementation.
//!
//! # References
//! * https://wiki.osdev.org/Interrupt_Descriptor_Table
//! * https://wiki.osdev.org/8259_PIC

use lazy_static::lazy_static;
use x86_64::structures::idt::InterruptDescriptorTable;

use crate::interrupts::{breakpoint_handler, pic::InterruptIndex};

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);

        unsafe {
            idt.double_fault
                .set_handler_fn(crate::interrupts::double_fault_handler)
                .set_stack_index(
                    crate::interrupts::gdt::DOUBLE_FAULT_IST_INDEX,
                );
        }

        idt[InterruptIndex::Timer.as_u8()]
            .set_handler_fn(crate::interrupts::timer_interrupt_handler);

        idt[InterruptIndex::Keyboard.as_u8()]
            .set_handler_fn(crate::interrupts::keyboard_interrupt_handler);

        idt.page_fault
            .set_handler_fn(crate::interrupts::page_fault_handler);

        idt
    };
}

pub fn init_idt() {
    IDT.load();
}
