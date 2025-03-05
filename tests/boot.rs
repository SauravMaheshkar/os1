#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(os1::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

use os1::println;

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    test_main();

    os1::hlt_loop();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    os1::test_panic_handler(info)
}

#[test_case]
fn test_println() {
    println!("test_println output");
}
