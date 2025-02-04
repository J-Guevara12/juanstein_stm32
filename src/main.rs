#![no_std]
#![no_main]

use defmt::*;

use embassy_executor::Spawner;

use embedded_graphics::{
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{Circle, PrimitiveStyleBuilder, Rectangle, Sector},
};

use {defmt_rtt as _, panic_probe as _};
mod display;
mod rcc;
mod spi;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("Hello World!");
    let mut config = embassy_stm32::Config::default();
    rcc::configure_rcc(&mut config);

    let p = embassy_stm32::init(config);

    let spi = spi::init(p.SPI1, p.PA5, p.PA7, p.DMA1_CH3);

    let mut buffer = [0_u8; 240];
    let mut display = display::create_display(spi, p.PC3, p.PC0, p.PC2, buffer.as_mut_slice());

    display
        .fill_solid(
            &Rectangle::new(Point::new(20, 20), Size::new(280, 200)),
            Rgb565::RED,
        )
        .unwrap();

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
