#![no_std]
#![cfg_attr(test, no_main)]
#![feature(abi_x86_interrupt)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use core::panic::PanicInfo;

pub mod interrupts;
pub mod io;
pub mod mem;

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

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        crate::print_serial!("{} ... ", core::any::type_name::<T>());
        self();
        crate::println_serial!("✓");
    }
}

pub fn test_runner(tests: &[&dyn Testable]) {
    crate::println_serial!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    crate::io::exit_qemu(crate::io::QemuExitCode::Success);
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    crate::println_serial!("x\n");
    crate::println_serial!("Error: {}\n", info);
    crate::io::exit_qemu(crate::io::QemuExitCode::Failed);
    hlt_loop();
}

#[cfg(test)]
use bootloader::{entry_point, BootInfo};

#[cfg(test)]
entry_point!(test_kernel_main);

#[cfg(test)]
fn test_kernel_main(_boot_info: &'static BootInfo) -> ! {
    init();
    test_main();
    hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}
