use core::sync::atomic::{AtomicU64, Ordering};

use x86_64::structures::idt::InterruptStackFrame;

use crate::drivers::apic::{self, registers::APICRegisters};

pub static TICKS: AtomicU64 = AtomicU64::new(0);

pub unsafe fn init(local_apic_ptr: *mut u32) {
    let svr = local_apic_ptr.offset(APICRegisters::Svr as isize / 4);
    svr.write_volatile(svr.read_volatile() | 0x100); // Set bit 8

    let lvt_lint1 = local_apic_ptr.offset(APICRegisters::LvtT as isize / 4);
    lvt_lint1.write_volatile(0x20 | (1 << 17)); // Vector 0x20, periodic mode

    let tdcr = local_apic_ptr.offset(APICRegisters::Tdcr as isize / 4);
    tdcr.write_volatile(0x1);

    let ticr = local_apic_ptr.offset(APICRegisters::Ticr as isize / 4);
    ticr.write_volatile(0x400);
}

pub extern "x86-interrupt" fn timer_handler(_stack_frame: InterruptStackFrame) {
    TICKS.fetch_add(1, Ordering::Relaxed);
    apic::end_interrupt();
}

#[inline]
pub fn get_ticks() -> u64 {
    TICKS.load(Ordering::Relaxed)
}
