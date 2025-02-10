use embassy_embedded_hal::shared_bus::asynch::spi::SpiDevice;
use embassy_stm32::{
    gpio::{Level, Output, Pin, Speed},
    mode::Async,
    spi::Spi,
};
use embassy_sync::{blocking_mutex::raw::NoopRawMutex, mutex::Mutex};
use mipidsi::{interface::SpiInterfaceAsync, models::ILI9341Rgb565, Display};
use static_cell::StaticCell;

pub const BUFFER_COLUMN_SIZE: usize = 80;

type SpiBus = Mutex<NoopRawMutex, Spi<'static, embassy_stm32::mode::Async>>;
pub type DisplayType<'a> = Display<
    SpiInterfaceAsync<
        'a,
        SpiDevice<'static, NoopRawMutex, Spi<'static, Async>, Output<'static>>,
        Output<'static>,
    >,
    ILI9341Rgb565,
    Output<'static>,
>;

pub async fn create_display<'a, P1: Pin, P2: Pin, P3: Pin>(
    spi: Spi<'static, Async>,
    reset_pin: P1,
    cs_pin: P2,
    dc_pin: P3,
    buffer: &'a mut [u8],
) -> DisplayType<'a> {
    static SPI_BUS: StaticCell<SpiBus> = StaticCell::new();
    let spi_bus = SPI_BUS.init(Mutex::new(spi));

    let reset = Output::new(reset_pin, Level::Low, Speed::VeryHigh);
    let cs = Output::new(cs_pin, Level::High, Speed::VeryHigh);
    let dc = Output::new(dc_pin, Level::Low, Speed::VeryHigh);

    let dev = SpiDevice::new(spi_bus, cs);
    let mut delay = embassy_time::Delay;

    let interface = SpiInterfaceAsync::new(dev, dc, buffer);

    mipidsi::Builder::new(ILI9341Rgb565, interface)
        .reset_pin(reset)
        .init(&mut delay)
        .await
        .unwrap()
}
