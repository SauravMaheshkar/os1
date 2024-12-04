//! Text User Interface Module
//!
//! This module provides a simple TUI.
//!
//! # References
//! * [Bare Bones - OSDev Wiki](https://wiki.osdev.org/Bare_Bones)

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
fn vga_entry_color(foreground: VgaColor, background: VgaColor) -> u8 {
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
fn vga_entry(uc: u8, color: u8) -> u16 {
    uc as u16 | (color as u16) << 8
}

/// The Terminal Writer is responsible for writing to the VGA buffer.
pub struct TerminalWriter {
    row: usize,
    column: usize,
    color: u8,
    buffer: *mut u16,
    vga_height: usize,
    vga_width: usize,
}

impl TerminalWriter {
    /// Creates a [`TerminalWriter`] with a VGA buffer filled with spaces `' '`.
    ///
    /// * Default color is `LightGray` on `Black`.
    /// * Default `vga_height` is 25.
    /// * Default `vga_width` is 80.
    pub fn new() -> TerminalWriter {
        let vga_height: usize = 25;
        let vga_width: usize = 80;
        let row = 0;
        let column = 0;
        let color = vga_entry_color(VgaColor::LightGray, VgaColor::Black);

        // VGA text mode buffer is located at 0xb8000
        // Table 1.1: VGA Text Modes
        // Source: https://web.archive.org/web/20150816220334/http://www.eyetap.org/cyborgs/manuals/soft_vga.pdf
        let buffer = 0xb8000 as *mut u16;

        // Fill the VGA buffer with spaces
        for y in 0..vga_height {
            for x in 0..vga_width {
                let index = y * vga_width + x;
                unsafe {
                    *buffer.add(index) = vga_entry(b' ', color);
                }
            }
        }

        TerminalWriter {
            row,
            column,
            color,
            buffer,
            vga_height,
            vga_width,
        }
    }

    /// Put a character at a specific position in the VGA buffer.
    ///
    /// # Arguments
    /// * `c` - The Unicode character as a `u8`
    /// * `color` - The color from [`VgaColor`]
    /// * `x` - The x position in the VGA buffer as a `usize`
    /// * `y` - The y position in the VGA buffer as a `usize`
    fn putentryat(&mut self, c: u8, color: u8, x: usize, y: usize) {
        let index = y * self.vga_width + x;
        unsafe {
            *self.buffer.add(index) = vga_entry(c, color);
        }
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
            self.column = 0;
            self.row += 1;
            if self.row == self.vga_height {
                self.row = 0;
            }
            return;
        }

        self.putentryat(c, self.color, self.column, self.row);
        self.column += 1;

        // wrap around
        if self.column == self.vga_width {
            self.column = 0;
            self.row += 1;
            if self.row == self.vga_height {
                self.row = 0;
            }
        }
    }
}
