#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(os1::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod io;

use core::panic::PanicInfo;

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    #[cfg(test)]
    test_main();

    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("[PANIC]: {}\n", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    os1::test_panic_handler(info)
}
