use crate::display::{self};
use embassy_stm32::{gpio::AnyPin, mode::Async, spi::Spi};
use embassy_time::{Duration, Ticker};
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::{Point, RgbColor, Size, WebColors};
use embedded_graphics::primitives::Rectangle;

use defmt::*;

use crate::display::BUFFER_COLUMN_SIZE;
use crate::game::player::Player;

const GOAL_FPS: u64 = 20;

/// The  GOal of the FPSs
///
/// # Arguments
///
/// * `spi` - Tables
/// * `reset_pin` -  
/// * `cs_pin` - [TODO:description]
/// * `dc_pin` - [TODO:description]
///
/// # Examples
///
/// ```
/// [TODO:write some example code]
/// ```
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

    let _area = Rectangle::new(Point::new(0, 0), Size::new(240, BUFFER_COLUMN_SIZE as u32));

    //let mut player = Player::new(1.0, 1.0);

    loop {
        for i in 0..crate::WIDTH / BUFFER_COLUMN_SIZE {
            let area = Rectangle::new(
                Point::new(0, (i * BUFFER_COLUMN_SIZE) as i32),
                Size::new(240, BUFFER_COLUMN_SIZE as u32),
            );
            let subarea = Rectangle::new(Point::new(20, 20), Size::new(200, 40));
            display.set_context(&area).await.unwrap();
            display
                .fill_solid_in_context(&_area, Rgb565::CSS_DARK_RED)
                .await
                .unwrap();
            display
                .fill_solid_in_context(&subarea, Rgb565::CSS_DARK_BLUE)
                .await
                .unwrap();
            display.flush().await.unwrap();
        }
        ticker.next().await;
    }
}
