use bootloader_api::info::{FrameBuffer, PixelFormat};
use embedded_graphics::pixelcolor::{Rgb888, RgbColor};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

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
