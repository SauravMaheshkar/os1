#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

pub mod io;

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
    loop {}
}

/// Entry point for `cargo xtest`
#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}
