#![no_std]
#![no_main]

use core::cell::RefCell;
use defmt::*;

use embassy_embedded_hal::shared_bus::blocking::spi::SpiDevice;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_stm32::spi::{Config, Spi};
use embassy_stm32::time::Hertz;

use embassy_sync::blocking_mutex::NoopMutex;
use embedded_graphics::{
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{Circle, PrimitiveStyleBuilder, Rectangle, Sector},
};

use mipidsi::models::ILI9341Rgb565;
use static_cell::StaticCell;

use {defmt_rtt as _, panic_probe as _};

type Spi3Bus = NoopMutex<RefCell<Spi<'static, embassy_stm32::mode::Async>>>;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("Hello World!");
    let mut config = embassy_stm32::Config::default();
    {
        use embassy_stm32::rcc::*;
        config.rcc.sys = Sysclk::PLL1_R;
        config.rcc.hse = Some(Hse {
            freq: Hertz::mhz(8),
            mode: HseMode::Oscillator,
        });
        config.rcc.pll = Some(Pll {
            source: PllSource::HSE,
            prediv: PllPreDiv::DIV1,
            mul: PllMul::MUL20,
            divp: None,
            divq: None,
            divr: Some(PllRDiv::DIV2),
        })
    }
    let p = embassy_stm32::init(config);

    let mut spi_config = Config::default();
    spi_config.frequency = Hertz(40_000_000);

    let spi = Spi::new_txonly(p.SPI1, p.PA5, p.PA7, p.DMA1_CH3, spi_config);

    let dc = Output::new(p.PC2, Level::Low, Speed::VeryHigh);
    let reset = Output::new(p.PC3, Level::Low, Speed::VeryHigh);
    let cs = Output::new(p.PC0, Level::High, Speed::VeryHigh);

    static SPI_BUS: StaticCell<Spi3Bus> = StaticCell::new();
    let spi_bus = SPI_BUS.init(NoopMutex::new(RefCell::new(spi)));

    let dev = SpiDevice::new(spi_bus, cs);
    let mut delay = embassy_time::Delay;

    let mut buffer = [0_u8; 240 * 3];
    let interface = mipidsi::interface::SpiInterface::new(dev, dc, &mut buffer);
    let mut display = mipidsi::Builder::new(ILI9341Rgb565, interface)
        .reset_pin(reset)
        .init(&mut delay)
        .unwrap();

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
