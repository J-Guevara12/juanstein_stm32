use defmt::{error, info};
use embassy_stm32::adc::AdcChannel;
use embassy_stm32::i2c::I2c;
use embassy_stm32::mode::Async;
use embassy_stm32::peripherals::{ADC1, PC0, PC1};
use embassy_time::{Duration, Ticker};

use crate::peripherals::adc::init;
use crate::PLAYER;

const GOAL_SAMPLE_FREQUENCY_HZ: u64 = 40;

#[embassy_executor::task]
pub async fn sensors_task(adc: ADC1, channel_x: PC0, channel_y: PC1, i2c: I2c<'static, Async>) {
    let mut adc = init(adc);
    adc.set_resolution(embassy_stm32::adc::Resolution::BITS12);
    let mut channel_x = channel_x.degrade_adc();
    let mut channel_y = channel_y.degrade_adc();

    let mut mpu = mpu6050::Mpu6050::new_with_sens(
        i2c,
        mpu6050::device::AccelRange::G2,
        mpu6050::device::GyroRange::D250,
    );
    mpu.init(&mut embassy_time::Delay);

    let mut ticker = Ticker::every(Duration::from_millis(1000 / GOAL_SAMPLE_FREQUENCY_HZ));
    let gpioc_ascr = 0x4800082C as *mut u32;
    unsafe { *gpioc_ascr = 0x3 }

    loop {
        {
            let x = ((adc.blocking_read(&mut channel_x) >> 8) as i32 - 7) as f32
                / (GOAL_SAMPLE_FREQUENCY_HZ as f32 * 16.0);
            let y = ((adc.blocking_read(&mut channel_y) >> 8) as i32 - 7) as f32
                / (GOAL_SAMPLE_FREQUENCY_HZ as f32 * 16.0);
            let mut player = PLAYER.lock().await;
            player.move_player(y, -x);
            match mpu.get_gyro() {
                Ok(gyro) => {
                    player.rotate_player(gyro[0]);
                }
                Err(_) => {
                    error!("I2C Error")
                }
            };
        }
        ticker.next().await;
    }
}
