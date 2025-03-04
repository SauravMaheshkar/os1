//! # Serial Communication
//!
//! * Ref: https://wiki.osdev.org/Serial_Ports

use lazy_static::lazy_static;
use spin::Mutex;
use uart_16550::SerialPort;

lazy_static! {
    pub static ref SERIAL1: Mutex<SerialPort> = {
        let mut serial_port = unsafe { SerialPort::new(0x3F8) };
        serial_port.init();
        Mutex::new(serial_port)
    };
}

#[doc(hidden)]
pub fn _print(args: ::core::fmt::Arguments) {
    use core::fmt::Write;
    SERIAL1
        .lock()
        .write_fmt(args)
        .expect("Printing to serial failed");
}

/// Prints to the host through the serial interface.
#[macro_export]
macro_rules! print_serial {
    ($($arg:tt)*) => {
        $crate::io::serial::_print(format_args!($($arg)*));
    };
}

/// Prints to the host through the serial interface, appending a newline.
#[macro_export]
macro_rules! println_serial {
    () => ($crate::print_serial!("\n"));
    ($fmt:expr) => ($crate::print_serial!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::print_serial!(
        concat!($fmt, "\n"), $($arg)*));
}
