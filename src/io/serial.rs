//! # Serial Communication
//!
//! * Ref: https://wiki.osdev.org/Serial_Ports
use core::arch::asm;

const PORT: u16 = 0x3f8;

unsafe fn inb(addr: u16) -> u8 {
    let mut ret;

    asm!(r#"
        .att_syntax
        in %dx, %al
        "#,
        in("dx") addr,
        out("al") ret
    );

    ret
}

unsafe fn outb(addr: u16, val: u8) {
    asm!(r#"
        .att_syntax
        out %al, %dx
        "#,
        in("dx") addr,
        in("al") val
    );
}

/// Terminate QEMU from within kernel main
///
/// * exit port defined in `.cargo/qemu_wrapper.sh`
///
/// # Example Invocation
/// ```rust
/// pub ... kernel_main (...) {
/// io::serial::exit(0);
/// }
/// ```
pub unsafe fn exit(code: u8) {
    const EXIT_PORT: u16 = 0xf4;
    outb(EXIT_PORT, code);
}

unsafe fn is_transit_empty() -> u8 {
    inb(PORT + 5) & 0x20
}

unsafe fn write_serial(a: u8) {
    while is_transit_empty() == 0 {}
    outb(PORT, a);
}

pub struct Serial {}

pub static SERIAL: Serial = Serial {};

#[derive(Debug)]
pub struct SerialInitError;

impl Serial {
    pub unsafe fn init() -> Result<(), SerialInitError> {
        outb(PORT + 1, 0x00); // Disable all interrupts
        outb(PORT + 3, 0x80); // Enable DLAB (set baud rate divisor)
        outb(PORT, 0x03); // Set divisor to 3 (low byte) 38400 baud
        outb(PORT + 1, 0x00); // (high byte)
        outb(PORT + 3, 0x03); // 8 bits, no parity, one stop bit
        outb(PORT + 2, 0xC7); // Enable FIFO, clear them, with 14-byte threshold
        outb(PORT + 4, 0x0B); // IRQs enabled, RTS/DSR set
        outb(PORT + 4, 0x1E); // Set in loopback mode, test the serial chip
        outb(PORT, 0xAE); // Test serial chip (send byte 0xAE and check if serial returns same
                          // byte)

        // Check if serial is faulty (i.e: not same byte as sent)
        if inb(PORT) != 0xAE {
            return Err(SerialInitError);
        }

        outb(PORT + 4, 0x0F);

        Ok(())
    }
}

impl core::fmt::Write for Serial {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        unsafe {
            for b in s.as_bytes() {
                write_serial(*b);
            }
        }
        Ok(())
    }
}
