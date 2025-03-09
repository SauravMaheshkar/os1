use bootloader_api::info::FrameBuffer;
use embedded_graphics::{
    image::Image,
    pixelcolor::Rgb888,
    prelude::{Drawable, Point},
};
use tinytga::Tga;

use crate::graphics::display::Display;

pub fn draw_tga(frame_buffer: &mut FrameBuffer) {
    let mut display = Display::new(frame_buffer);
    let data = include_bytes!("./snoopy.tga");
    let tga: Tga<Rgb888> = Tga::from_slice(data).unwrap();

    let size = tga.as_raw().size();
    let image_width = size.width as usize;
    let image_height = size.height as usize;

    let screen_width = display.framebuffer().info().width;
    let screen_height = display.framebuffer().info().height;

    // Center the image on screen
    let center_x = (screen_width / 2 - image_width / 2) as i32;
    let center_y = (screen_height / 2 - image_height / 2) as i32;

    // Draw the image only once at the center
    Image::new(&tga, Point::new(center_x, center_y))
        .draw(&mut display)
        .unwrap();
}
