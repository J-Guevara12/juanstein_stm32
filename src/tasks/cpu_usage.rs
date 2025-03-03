use crate::SLEEP_TICKS;
use embassy_time::{Duration, Instant, Ticker};
use portable_atomic::Ordering;

#[embassy_executor::task]
pub async fn cpu_usage() {
    let mut previous_tick = 0u64;
    let mut previous_sleep_tick = 0u64;
    let mut ticker = Ticker::every(Duration::from_millis(1000));

    loop {
        let current_tick = Instant::now().as_ticks();
        let current_sleep_tick = SLEEP_TICKS.load(Ordering::Relaxed);

        let sleep_tick_difference = (current_sleep_tick - previous_sleep_tick) as f32;
        let tick_difference = (current_tick - previous_tick) as f32;

        let usage = 1f32 - sleep_tick_difference / tick_difference;

        previous_tick = current_tick;
        previous_sleep_tick = current_sleep_tick;
        defmt::info!("Cpu usage: {}%", usage * 100f32);
        ticker.next().await;
    }
}
