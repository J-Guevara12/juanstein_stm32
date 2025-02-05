use crate::display;
use embassy_stm32::{gpio::AnyPin, mode::Async, spi::Spi};
use embassy_time::{Duration, Ticker};
use embedded_graphics::{pixelcolor::Rgb565, prelude::*};

const GOAL_FPS: u64 = 10;

#[embassy_executor::task]
pub async fn display_task(
    spi: Spi<'static, Async>,
    reset_pin: AnyPin,
    cs_pin: AnyPin,
    dc_pin: AnyPin,
) {
    let mut ticker = Ticker::every(Duration::from_millis(1000 / GOAL_FPS));
    let mut buffer = [0_u8; 240];
    let mut display =
        display::create_display(spi, reset_pin, cs_pin, dc_pin, buffer.as_mut_slice());
    let mut color = Rgb565::BLACK;

    loop {
        display.clear(color).unwrap();
        ticker.next().await;

        if color == Rgb565::BLACK {
            color = Rgb565::WHITE;
        } else {
            color = Rgb565::BLACK;
        }
    }
}
