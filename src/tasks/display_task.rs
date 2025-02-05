use crate::display;
use embassy_stm32::{gpio::AnyPin, mode::Async, spi::Spi};
use embedded_graphics::{pixelcolor::Rgb565, prelude::*, primitives::*};

#[embassy_executor::task]
pub async fn display_task(
    spi: Spi<'static, Async>,
    reset_pin: AnyPin,
    cs_pin: AnyPin,
    dc_pin: AnyPin,
) {
    let mut buffer = [0_u8; 240];
    let mut display =
        display::create_display(spi, reset_pin, cs_pin, dc_pin, buffer.as_mut_slice());

    let mut progress: i32 = 0;
    const STEPS: i32 = 10;

    let sector_style = PrimitiveStyleBuilder::new()
        .stroke_color(Rgb565::BLACK)
        .stroke_width(2)
        .fill_color(Rgb565::YELLOW)
        .build();
    let eye_style = PrimitiveStyleBuilder::new()
        .stroke_color(Rgb565::BLACK)
        .stroke_width(1)
        .fill_color(Rgb565::BLACK)
        .build();

    display.clear(Rgb565::WHITE).unwrap();
    loop {
        let p = (progress - STEPS).abs();

        // Draw a Sector as the main Pacman feature.
        Sector::new(
            Point::new(2, 2),
            61,
            Angle::from_degrees((p * 30 / STEPS) as f32),
            Angle::from_degrees((360 - 2 * p * 30 / STEPS) as f32),
        )
        .into_styled(sector_style)
        .draw(&mut display)
        .unwrap();

        // Draw a Circle as the eye.
        Circle::new(Point::new(36, 16), 5)
            .into_styled(eye_style)
            .draw(&mut display)
            .unwrap();

        progress = (progress + 1) % (2 * STEPS + 1);
    }
}
