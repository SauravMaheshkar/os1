use x86_64::structures::idt::InterruptStackFrame;

use crate::drivers::apic::{self, registers::APICRegisters};

pub unsafe fn init(local_apic_ptr: *mut u32) {
    let svr = local_apic_ptr.offset(APICRegisters::Svr as isize / 4);
    svr.write_volatile(svr.read_volatile() | 0x100); // Set bit 8

    let lvt_lint1 = local_apic_ptr.offset(APICRegisters::LvtT as isize / 4);
    lvt_lint1.write_volatile(0x20 | (1 << 17)); // Vector 0x20, periodic mode

    let tdcr = local_apic_ptr.offset(APICRegisters::Tdcr as isize / 4);
    tdcr.write_volatile(0x3); // Divide by 16 mode

    let ticr = local_apic_ptr.offset(APICRegisters::Ticr as isize / 4);
    ticr.write_volatile(0x100000); // An arbitrary value for the initial value
                                   // of the timer
}

pub extern "x86-interrupt" fn timer_handler(_stack_frame: InterruptStackFrame) {
    apic::end_interrupt();
}
