use crate::display;
use embassy_stm32::{gpio::AnyPin, mode::Async, spi::Spi};
use embassy_time::{Duration, Ticker};
use embedded_graphics::{pixelcolor::Rgb565, prelude::*, primitives::Rectangle};

const GOAL_FPS: u64 = 30;
const BUFFER_COLUMN_SIZE: usize = 80;
const SCREEN_WIDTH: usize = 320;

#[embassy_executor::task]
pub async fn display_task(
    spi: Spi<'static, Async>,
    reset_pin: AnyPin,
    cs_pin: AnyPin,
    dc_pin: AnyPin,
) {
    let mut ticker = Ticker::every(Duration::from_millis(1000 / GOAL_FPS));
    let mut buffer = [0_u8; 2 * 240 * BUFFER_COLUMN_SIZE];
    let mut display =
        display::create_display(spi, reset_pin, cs_pin, dc_pin, buffer.as_mut_slice()).await;
    let mut color = Rgb565::BLACK;

    let mut counter = 0;
    loop {
        for x in 0..=(SCREEN_WIDTH / BUFFER_COLUMN_SIZE) {
            let area = Rectangle::new(
                Point::new(0, (x * BUFFER_COLUMN_SIZE) as i32),
                Size::new(240, BUFFER_COLUMN_SIZE as u32),
            );
            display.fill_solid(&area, color).await.unwrap();
            display.flush().await.unwrap();
        }

        if counter % GOAL_FPS == 0 {
            if color == Rgb565::BLACK {
                color = Rgb565::WHITE;
            } else {
                color = Rgb565::BLACK;
            }
        }
        counter = (counter + 1) % (GOAL_FPS * 2);
        ticker.next().await;
    }
}
