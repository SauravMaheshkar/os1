use bootloader_api::info::FrameBuffer;
use embedded_graphics::{
    pixelcolor::Rgb888,
    prelude::*,
    primitives::{Circle, PrimitiveStyle},
};

use crate::graphics::display::Display;

pub async fn bouncing_ball(frame_buffer: &mut FrameBuffer) {
    let mut display = Display::new(frame_buffer);

    let width = display.size().width as i32;
    let height = display.size().height as i32;

    let mut x = 30;
    let mut y = 50;
    let mut dx = 75;
    let mut dy = 75;
    const RADIUS: i32 = 50;

    loop {
        Circle::new(Point::new(x, y), RADIUS as u32 * 2)
            .into_styled(PrimitiveStyle::with_fill(Rgb888::BLACK))
            .draw(&mut display)
            .unwrap();

        x += dx;
        y += dy;

        if (x + RADIUS) > width || (x - RADIUS) < 0 {
            dx *= -1;
        }
        if (y + RADIUS) > height || (y - RADIUS) < 0 {
            dy *= -1;
        }

        Circle::new(Point::new(x, y), RADIUS as u32 * 2)
            .into_styled(PrimitiveStyle::with_fill(Rgb888::BLUE))
            .draw(&mut display)
            .unwrap();

        for _ in 0..100000 {}
    }
}
