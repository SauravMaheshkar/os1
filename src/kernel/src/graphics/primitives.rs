//! Primitives for drawing on the framebuffer.
use bootloader_api::info::{FrameBuffer, PixelFormat};
use embedded_graphics::pixelcolor::{Rgb888, RgbColor};

/// A position in the framebuffer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    /// The x coordinate.
    pub x: usize,
    /// The y coordinate.
    pub y: usize,
}

/// Set a pixel in the framebuffer.
///
/// supports RGB, BGR, and grayscale pixel formats.
///
/// # Arguments
/// * `framebuffer` - The framebuffer to draw on.
/// * `position` - The position of the pixel as a [`Position`].
/// * `color` - The color of the pixel as a [`Rgb888`] color.
pub fn set_pixel_in(
    framebuffer: &mut FrameBuffer,
    position: Position,
    color: Rgb888,
) {
    let info = framebuffer.info();

    let byte_offset = {
        let line_offset = position.y * info.stride;
        let pixel_offset = line_offset + position.x;
        pixel_offset * info.bytes_per_pixel
    };

    let pixel_buffer = &mut framebuffer.buffer_mut()[byte_offset..];
    match info.pixel_format {
        PixelFormat::Rgb => {
            pixel_buffer[0] = color.r();
            pixel_buffer[1] = color.g();
            pixel_buffer[2] = color.b();
        }
        PixelFormat::Bgr => {
            pixel_buffer[0] = color.b();
            pixel_buffer[1] = color.g();
            pixel_buffer[2] = color.r();
        }
        PixelFormat::U8 => {
            let gray = color.r() / 3 + color.g() / 3 + color.b() / 3;
            pixel_buffer[0] = gray;
        }
        other => panic!("unknown pixel format {other:?}"),
    }
}
