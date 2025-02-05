#![no_std]
#![no_main]

use defmt::*;

use cortex_m_rt::entry;
use embassy_executor::raw::Executor as RawExecutor;
use embassy_time::Instant;
use portable_atomic::{AtomicU64, Ordering};
use static_cell::StaticCell;

use {defmt_rtt as _, panic_probe as _};

mod display;
mod rcc;
mod spi;
mod tasks;

static EXECUTOR: StaticCell<RawExecutor> = StaticCell::new();
static SLEEP_TICKS: AtomicU64 = AtomicU64::new(0);

#[entry]
fn main() -> ! {
    info!("Hello World!");
    let mut config = embassy_stm32::Config::default();
    rcc::configure_rcc(&mut config);

    let p = embassy_stm32::init(config);

    let spi = spi::init(p.SPI1, p.PA5, p.PA7, p.DMA1_CH3);
    let executor = EXECUTOR.init(RawExecutor::new(usize::MAX as *mut ()));
    let spawner = executor.spawner();
    spawner
        .spawn(tasks::display_task(
            spi,
            p.PC3.into(),
            p.PC0.into(),
            p.PC2.into(),
        ))
        .unwrap();
    spawner.spawn(tasks::cpu_usage()).unwrap();
    loop {
        let before = Instant::now().as_ticks();
        cortex_m::asm::wfe();
        let after = Instant::now().as_ticks();
        SLEEP_TICKS.fetch_add(after - before, Ordering::Relaxed);
        unsafe { executor.poll() };
    }
}
