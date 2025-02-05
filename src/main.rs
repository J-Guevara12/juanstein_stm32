#![no_std]
#![no_main]

use defmt::*;

use embassy_executor::Spawner;

use {defmt_rtt as _, panic_probe as _};

mod display;
mod rcc;
mod spi;
mod tasks;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    info!("Hello World!");
    let mut config = embassy_stm32::Config::default();
    rcc::configure_rcc(&mut config);

    let p = embassy_stm32::init(config);

    let spi = spi::init(p.SPI1, p.PA5, p.PA7, p.DMA1_CH3);

    spawner
        .spawn(tasks::display_task(
            spi,
            p.PC3.into(),
            p.PC0.into(),
            p.PC2.into(),
        ))
        .unwrap();
}
