use crate::peripherals::adc::init;
use ::embassy_stm32::peripherals::{ADC1, PC0, PC1};
use defmt::info;
use embassy_stm32::adc::AdcChannel;
use embassy_time::{Duration, Ticker};

const GOAL_SAMPLE_FREQUENCY_HZ: u64 = 40;

#[embassy_executor::task]
pub async fn adc_task(adc: ADC1, channel_x: PC0, channel_y: PC1) {
    let mut adc = init(adc);
    let mut channel_x = channel_x.degrade_adc();
    let mut channel_y = channel_y.degrade_adc();

    let mut ticker = Ticker::every(Duration::from_millis(1000 / GOAL_SAMPLE_FREQUENCY_HZ));
    let gpioc_ascr = 0x4800082C as *mut u32;
    unsafe { *gpioc_ascr = 0x3 }

    loop {
        info!(
            "({}, {})",
            adc.blocking_read(&mut channel_x),
            adc.blocking_read(&mut channel_y)
        );
        ticker.next().await;
    }
}
