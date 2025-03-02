//! Custom Macros
//!
//! Utilises the `core::fmt::Write` trait

#[macro_export]
macro_rules! print_vga {
    ($($arg: tt)*) => {
        #[allow(unused_unsafe)]
        unsafe {
            use core::fmt::Write as FmtWrite;
            let mut sinks = $crate::io::SINKS.borrow_mut();
            if let Some(vga) = &mut sinks.vga {
                write!(vga, $($arg)*).expect("Failed to print to vga");
            }
        }
    };
}

#[macro_export]
macro_rules! print_serial {
    ($($arg: tt)*) => {
        #[allow(unused_unsafe)]
        unsafe {
            use core::fmt::Write as FmtWrite;
            let mut sinks = $crate::io::SINKS.borrow_mut();
            if let Some(serial) = &mut sinks.serial {
                write!(serial, $($arg)*).expect("Failed to print to serial");
            }
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
