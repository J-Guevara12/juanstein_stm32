use embassy_stm32::mode::Async;
use embassy_stm32::spi::{Config, Spi};
use embassy_stm32::time::Hertz;

const SPI_FREQUENCY: u32 = 40_000_000;

pub fn init<T, U, V, W>(spi: T, sck: V, mosi: U, tx_dma: W) -> Spi<'static, Async>
where
    T: embassy_stm32::spi::Instance,
    U: embassy_stm32::spi::MosiPin<T>,
    V: embassy_stm32::spi::SckPin<T>,
    W: embassy_stm32::spi::TxDma<T>,
{
    let mut spi_config = Config::default();
    spi_config.frequency = Hertz(SPI_FREQUENCY);

    Spi::new_txonly(spi, sck, mosi, tx_dma, spi_config)
}
