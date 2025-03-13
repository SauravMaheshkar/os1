//! Ah, ah, ah! You didn't say the magic word!
extern crate alloc;

use alloc::vec::Vec;

use bootloader_api::info::FrameBuffer;
use embedded_graphics::{
    image::ImageDrawable,
    pixelcolor::Rgb888,
    prelude::{DrawTargetExt, Point},
};

use crate::{devices::timer::get_ticks, graphics::display::Display};

/// Display the Nedry gif.
pub async fn magic_word(frame_buffer: &mut FrameBuffer) {
    let mut display = Display::new(frame_buffer);
    let data = include_bytes!("./nedry.gif");

    let gif = tinygif::Gif::<Rgb888>::from_slice(data).unwrap();
    let frames: Vec<_> = gif.frames().collect();
    let mut last_frame_time = get_ticks();
    let mut current_frame = 0;

    let screen_width = display.framebuffer().info().width as u32;
    let screen_height = display.framebuffer().info().height as u32;

    let gif_width = gif.width() as u32;
    let gif_height = gif.height() as u32;

    let x_offset = (screen_width - gif_width) / 2;
    let y_offset = (screen_height - gif_height) / 2;

    loop {
        let frame = &frames[current_frame];

        frame
            .draw(
                &mut display
                    .translated(Point::new(x_offset as i32, y_offset as i32)),
            )
            .unwrap();

        let delay = (frame.delay_centis as u64) * 10;

        let now = get_ticks();
        let elapsed_ticks = now - last_frame_time;

        if elapsed_ticks >= delay {
            current_frame = (current_frame + 1) % frames.len();
            last_frame_time = now;
        }
    }
}
