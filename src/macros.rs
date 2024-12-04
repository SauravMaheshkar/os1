//! Custom Macros to write to the VGA buffer
//!
//! Utilises the `core::fmt::Write` trait

#[macro_export]
macro_rules! print {
    ($($arg: tt)*) => {
        #[allow(invalid_reference_casting)]
        unsafe {
            use $crate::tui::TerminalWriter;
            use core::fmt::Write as FmtWrite;
            let writer = &$crate::tui::TERMINAL as *const TerminalWriter;
            let writer = writer as *mut TerminalWriter;
            write!(&mut *(writer), $($arg)*).expect("failed to write to VGA buffer");
        }
    };
}

#[macro_export]
macro_rules! println {
    ($($arg: tt)*) => {{
        print!($($arg)*);
        print!("\n");
    }};
}

#[macro_export]
macro_rules! println_str {
    ($fmt:expr, $ptr:expr) => {
        let c_str = unsafe {
            core::ffi::CStr::from_ptr($ptr as *const i8)
                .to_str()
                .expect("Invalid UTF-8 string")
        };
        println!($fmt, c_str);
    };
}
