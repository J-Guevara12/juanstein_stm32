use crate::display::{self};
use embassy_stm32::{gpio::AnyPin, mode::Async, spi::Spi};
use embassy_time::{Duration, Ticker};

use crate::display::BUFFER_COLUMN_SIZE;
use crate::game::player::Player;

const GOAL_FPS: u64 = 20;

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

    let mut player = Player::new(1.0, 1.0);

    loop {
        player.draw_player_raycaster(&mut display).await;
        player.theta = (player.theta + 0.02) % (crate::PI * 2.0);
        ticker.next().await;
    }
}
