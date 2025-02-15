//! Custom Macros to write to the VGA buffer
//!
//! Utilises the `core::fmt::Write` trait

#[macro_export]
macro_rules! print_vga {
    ($($arg: tt)*) => {
        #[allow(invalid_reference_casting)]
        unsafe {
            use $crate::io::vga::TerminalWriter;
            use core::fmt::Write as FmtWrite;

            let vga_writer = &$crate::io::vga::TERMINAL as *const TerminalWriter;
            let vga_writer = vga_writer as *mut TerminalWriter;
            write!(&mut *(vga_writer), $($arg)*).expect("failed to write to VGA buffer");
        }
    };
}

#[macro_export]
macro_rules! print_serial {
    ($($arg: tt)*) => {
        #[allow(invalid_reference_casting)]
        unsafe {
            use $crate::io::serial::Serial;
            use core::fmt::Write as FmtWrite;

            let serial_writer = &$crate::io::serial::SERIAL as *const Serial;
            let serial_writer = serial_writer as *mut Serial;
            write!(&mut *(serial_writer), $($arg)*).expect("failed to write to serial port");
        }
    };
}

#[macro_export]
macro_rules! println_vga {
    ($($arg: tt)*) => {{
        print_vga!($($arg)*);
        print_vga!("\n");
    }};
}

#[macro_export]
macro_rules! println_serial {
    ($($arg: tt)*) => {{
        print_serial!($($arg)*);
        print_serial!("\n");
    }};
}
