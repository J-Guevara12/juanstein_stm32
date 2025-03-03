use crate::display::{self};
use crate::PLAYER;
use embassy_stm32::{gpio::AnyPin, mode::Async, spi::Spi};
use embassy_time::{Duration, Ticker};

use crate::display::BUFFER_COLUMN_SIZE;

const GOAL_FPS: u64 = 30;

/// Updates the display at GOAL_FPS HZ
/// Copies the player from the Mutex and calls tha draw_player_raycaster method
///
/// # Arguments
///
/// * `spi` - SPI Object
/// * `reset_pin` - PIN connected to reset
/// * `cs_pin` - PIN connected to CS
/// * `dc_pin` - PIN connected to DC
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

    loop {
        let local_player;
        {
            local_player = PLAYER.lock().await.clone();
        }
        local_player.draw_player_raycaster(&mut display).await;
        ticker.next().await;
    }
}
