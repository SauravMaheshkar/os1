#![no_std]
#![no_main]

use core::panic::PanicInfo;

use os1::{
    io::{exit_qemu, QemuExitCode},
    print_serial, println_serial,
};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    should_fail();
    println_serial!("[test did not panic]");
    exit_qemu(QemuExitCode::Failed);
    os1::hlt_loop();
}

fn should_fail() {
    print_serial!("panic::should_fail... ");
    assert_eq!(0, 1);
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println_serial!("✓");
    exit_qemu(QemuExitCode::Success);
    loop {}
}
