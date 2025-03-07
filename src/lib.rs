#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

extern crate alloc;

pub mod interrupts;
pub mod io;
pub mod mem;
pub mod task;

pub fn init() {
    interrupts::gdt::init_gdt();
    interrupts::idt::init_idt();
    unsafe {
        interrupts::pic::PICS.lock().initialize();
    }

    x86_64::instructions::interrupts::enable();
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
