#![no_std]
#![no_main]

mod game;
mod peripherals;
mod rendering;
mod tasks;
mod textures;

use defmt::*;

use cortex_m_rt::entry;
use embassy_executor::raw::Executor as RawExecutor;
use embassy_time::Instant;
use peripherals::{display, i2c, rcc, spi};
use portable_atomic::{AtomicU64, Ordering};
use static_cell::StaticCell;

use embassy_sync::{blocking_mutex::raw::ThreadModeRawMutex, mutex::Mutex};

use {defmt_rtt as _, panic_probe as _};

use game::player::Player;

static EXECUTOR: StaticCell<RawExecutor> = StaticCell::new();
static SLEEP_TICKS: AtomicU64 = AtomicU64::new(0);

const HEIGHT: usize = 240;
const WIDTH: usize = 320;
pub const PI: f32 = 3.14159265359;

static PLAYER: Mutex<ThreadModeRawMutex, Player> = Mutex::new(Player::default());

#[entry]
fn main() -> ! {
    info!("Hello World!");
    let mut config = embassy_stm32::Config::default();
    rcc::configure_rcc(&mut config);

    let p = embassy_stm32::init(config);

    let spi = spi::init(p.SPI1, p.PA5, p.PA7, p.DMA1_CH3);
    let i2c = i2c::init(p.I2C1, p.PB8, p.PB9, p.DMA1_CH6, p.DMA1_CH7);

    let executor = EXECUTOR.init(RawExecutor::new(usize::MAX as *mut ()));
    let spawner = executor.spawner();

    spawner
        .spawn(tasks::display_task(
            spi,
            p.PC3.into(),
            p.PB0.into(),
            p.PC2.into(),
        ))
        .unwrap();

    spawner.spawn(tasks::cpu_usage()).unwrap();
    spawner.spawn(tasks::sensors_task(p.ADC1, p.PC0, p.PC1, i2c));

    loop {
        let before = Instant::now().as_ticks();
        cortex_m::asm::wfe();
        let after = Instant::now().as_ticks();
        SLEEP_TICKS.fetch_add(after - before, Ordering::Relaxed);
        unsafe { executor.poll() };
    }
}
