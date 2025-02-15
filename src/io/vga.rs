//! VGA Module
//!
//! This module provides a simple interface to use VGA.
//!
//! # References
//! * [Bare Bones - OSDev Wiki](https://wiki.osdev.org/Bare_Bones)

use core::sync::atomic::Ordering::Relaxed;
use core::{
    fmt::Write,
    sync::atomic::{AtomicU8, AtomicUsize},
};

pub static TERMINAL: TerminalWriter = TerminalWriter::new();

const VGA_HEIGHT: usize = 25;
const VGA_WIDTH: usize = 80;

/// Enum for the different colors available in the TUI
///
/// The colors are based on the VGA color palette.
#[allow(dead_code)]
enum VgaColor {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

/// Set the foreground and background color for VGA Entry
///
/// # Arguments
/// * `foreground` - The foreground color from [`VgaColor`]
/// * `background` - The background color from [`VgaColor`]
///
/// # Returns
/// * A `u8` representing the color
const fn vga_entry_color(foreground: VgaColor, background: VgaColor) -> u8 {
    (foreground as u8) | (background as u8) << 4
}

/// Create a VGA Entry
///
/// # Arguments
/// * `uc` - The Unicode character as a `u8`
/// * `color` - The color as a `u8`
///
/// # Returns
/// * A `u16` representing the VGA Entry
const fn vga_entry(uc: u8, color: u8) -> u16 {
    uc as u16 | (color as u16) << 8
}

/// The Terminal Writer is responsible for writing to the VGA buffer.
pub struct TerminalWriter {
    cursor: AtomicUsize,
    color: AtomicU8,
    buffer: *mut u16,
}

impl TerminalWriter {
    /// Creates a [`TerminalWriter`] with a VGA buffer filled with spaces `' '`.
    ///
    /// * Default color is `LightGray` on `Black`.
    /// * Default `vga_height` is 25.
    /// * Default `vga_width` is 80.
    const fn new() -> TerminalWriter {
        let cursor = AtomicUsize::new(0);
        let color = AtomicU8::new(vga_entry_color(VgaColor::LightGray, VgaColor::Black));
        let buffer = 0xb8000 as *mut u16;

        TerminalWriter {
            cursor,
            color,
            buffer,
        }
    }

    pub fn init() -> &'static TerminalWriter {
        let color = TERMINAL.color.load(Relaxed);

        // Fill the VGA buffer with spaces
        for y in 0..VGA_HEIGHT {
            for x in 0..VGA_WIDTH {
                let index = y * VGA_WIDTH + x;
                unsafe {
                    *TERMINAL.buffer.add(index) = vga_entry(b' ', color);
                }
            }
        }

        &TERMINAL
    }

    /// Write a string to the VGA buffer.
    ///
    /// # Arguments
    /// * `data` - The string as a slice of `u8`
    pub fn write(&mut self, data: &[u8]) {
        for c in data {
            self.putchar(*c);
        }
    }

    /// Write a character to the VGA buffer.
    ///
    /// # Arguments
    /// * `c` - The Unicode character as a `u8`
    fn putchar(&mut self, c: u8) {
        // newline
        if c == b'\n' {
            let mut cursor = self.cursor.load(Relaxed);
            cursor += VGA_WIDTH - (cursor % VGA_WIDTH);
            self.cursor.store(cursor, Relaxed);
            return;
        }

        let color = self.color.load(Relaxed);
        let cursor = self.cursor.fetch_add(1, Relaxed);

        unsafe {
            *self.buffer.add(cursor) = vga_entry(c, color);
        }
    }
}

impl Write for TerminalWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write(s.as_bytes());
        Ok(())
    }
}

// Shared static variables must have a type that implements `Sync`
unsafe impl Sync for TerminalWriter {}
