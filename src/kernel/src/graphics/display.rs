//! Graphics display driver for the framebuffer.
//!
//! # References
//! * <https://github.com/ChocolateLoverRaj/code-runner/blob/main/kernel/src/frame_buffer.rs>

use bootloader_api::info::{FrameBuffer, PixelFormat};
use embedded_graphics::{
    draw_target::DrawTarget,
    geometry::{OriginDimensions, Size},
    pixelcolor::{Rgb888, RgbColor},
    prelude::{Dimensions, Point},
    Pixel,
};

use super::primitives::{set_pixel_in, Position};

/// A display that can draw to the framebuffer.
pub struct Display<'f> {
    /// The framebuffer to draw on.
    framebuffer: &'f mut FrameBuffer,
}

impl Display<'_> {
    /// Create a new display.
    pub fn new(framebuffer: &mut FrameBuffer) -> Display {
        Display { framebuffer }
    }

    /// Draw a pixel on the display.
    ///
    /// uses the [`set_pixel_in`] function to set the pixel in the framebuffer.
    ///
    /// # Arguments
    /// * `pixel` - The pixel to draw as a [`Pixel`].
    /// * `color` - The color of the pixel as a [`Rgb888`] color.
    fn draw_pixel(&mut self, Pixel(Point { x, y }, color): Pixel<Rgb888>) {
        set_pixel_in(
            self.framebuffer,
            Position {
                x: x as usize,
                y: y as usize,
            },
            color,
        );
    }

    /// Get a reference to the framebuffer.
    pub fn framebuffer(&self) -> &FrameBuffer {
        self.framebuffer
    }
}

impl DrawTarget for Display<'_> {
    /// The color type of the display.
    type Color = Rgb888;

    /// Drawing operations can never fail.
    type Error = core::convert::Infallible;

    /// Draw from a given iterator of pixels.
    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for pixel in pixels.into_iter() {
            self.draw_pixel(pixel);
        }

        Ok(())
    }

    /// Fill a contiguous area with a color.
    ///
    /// uses the [`set_pixel_in`] function to set the pixel in the framebuffer.
    ///
    /// # Arguments
    /// * `area` - The area to fill as a
    ///   [`embedded_graphics::primitives::Rectangle`].
    /// * `colors` - The colors to fill the area with as an iterator
    fn fill_contiguous<I>(
        &mut self,
        area: &embedded_graphics::primitives::Rectangle,
        colors: I,
    ) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Self::Color>,
    {
        let info = self.framebuffer.info();
        let buffer = self.framebuffer.buffer_mut();
        let draw_pixel =
            |pixel_buffer: &mut [u8], color: Self::Color| match info
                .pixel_format
            {
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
            };
        let mut colors = colors.into_iter();
        for y in area.top_left.y as usize
            ..(area.top_left.y as usize)
                .saturating_add(area.size.height as usize)
                .min(info.height)
        {
            for x in area.top_left.x as usize
                ..(area.top_left.x as usize)
                    .saturating_add(area.size.width as usize)
                    .min(info.width)
            {
                let start = y * info.stride + x;
                let pixel_buffer = &mut buffer[start * info.bytes_per_pixel
                    ..(start + 1) * info.bytes_per_pixel];
                draw_pixel(pixel_buffer, colors.next().unwrap());
            }
        }
        Ok(())
    }

    /// Fill the entire display with a color.
    ///
    /// # Arguments
    /// * `area` - The area to fill as a
    ///   [`embedded_graphics::primitives::Rectangle`].
    /// * `color` - The color to fill the display with.
    fn fill_solid(
        &mut self,
        area: &embedded_graphics::primitives::Rectangle,
        color: Self::Color,
    ) -> Result<(), Self::Error> {
        let info = self.framebuffer.info();

        // Early return
        if area.top_left.x >= info.width as i32
            || area.top_left.y >= info.height as i32
            || area.top_left.x + area.size.width as i32 <= 0
            || area.top_left.y + area.size.height as i32 <= 0
        {
            return Ok(());
        }

        let start_x = area.top_left.x.max(0) as usize;
        let start_y = area.top_left.y.max(0) as usize;
        let end_x = (area.top_left.x + area.size.width as i32)
            .min(info.width as i32) as usize;
        let end_y = (area.top_left.y + area.size.height as i32)
            .min(info.height as i32) as usize;

        if start_x >= end_x || start_y >= end_y {
            return Ok(());
        }

        match info.pixel_format {
            PixelFormat::Bgr => {
                let buffer = self.framebuffer.buffer_mut();

                for y in start_y..end_y {
                    for x in start_x..end_x {
                        let pixel_index =
                            (y * info.stride + x) * info.bytes_per_pixel;
                        if pixel_index + info.bytes_per_pixel <= buffer.len() {
                            buffer[pixel_index] = color.b();
                            buffer[pixel_index + 1] = color.g();
                            buffer[pixel_index + 2] = color.r();
                        }
                    }
                }
            }
            other => panic!("unknown pixel format {other:?}"),
        };
        Ok(())
    }

    /// Clear the display with a color.
    fn clear(&mut self, color: Self::Color) -> Result<(), Self::Error> {
        if color == Rgb888::BLACK {
            self.framebuffer.buffer_mut().fill(0);
        } else {
            self.fill_solid(&self.bounding_box(), color)?;
        }
        Ok(())
    }
}

impl OriginDimensions for Display<'_> {
    /// Get the dimensions of the display.
    fn size(&self) -> Size {
        let info = self.framebuffer.info();

        Size::new(info.width as u32, info.height as u32)
    }
}
