use crate::peripherals::adc::init;
use ::embassy_stm32::peripherals::{ADC1, PA0};
use defmt::info;
use embassy_stm32::adc::AdcChannel;
use embassy_time::{Duration, Ticker};

const GOAL_SAMPLE_FREQUENCY_HZ: u64 = 40;

#[embassy_executor::task]
pub async fn adc_task(adc: ADC1, channel: PA0) {
    let mut adc = init(adc);
    let mut channel = channel.degrade_adc();

    let mut ticker = Ticker::every(Duration::from_millis(1000 / GOAL_SAMPLE_FREQUENCY_HZ));

    loop {
        info!("{}", adc.blocking_read(&mut channel));
        ticker.next().await;
    }
}
