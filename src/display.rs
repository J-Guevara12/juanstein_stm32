use core::cell::RefCell;
use embassy_embedded_hal::shared_bus::blocking::spi::SpiDevice;
use embassy_stm32::{
    gpio::{Level, Output, Pin, Speed},
    mode::Async,
    spi::Spi,
};
use embassy_sync::blocking_mutex::{raw::NoopRawMutex, NoopMutex};
use mipidsi::{interface::SpiInterface, models::ILI9341Rgb565, Display};
use static_cell::StaticCell;

type SpiBus = NoopMutex<RefCell<Spi<'static, embassy_stm32::mode::Async>>>;

pub fn create_display<'a, P1: Pin, P2: Pin, P3: Pin>(
    spi: Spi<'static, Async>,
    reset_pin: P1,
    cs_pin: P2,
    dc_pin: P3,
    buffer: &'a mut [u8],
) -> Display<
    SpiInterface<
        'a,
        embassy_embedded_hal::shared_bus::blocking::spi::SpiDevice<
            'static,
            NoopRawMutex,
            embassy_stm32::spi::Spi<'static, embassy_stm32::mode::Async>,
            embassy_stm32::gpio::Output<'static>,
        >,
        embassy_stm32::gpio::Output<'static>,
    >,
    ILI9341Rgb565,
    embassy_stm32::gpio::Output<'static>,
> {
    static SPI_BUS: StaticCell<SpiBus> = StaticCell::new();
    let spi_bus = SPI_BUS.init(NoopMutex::new(RefCell::new(spi)));

    let reset = Output::new(reset_pin, Level::Low, Speed::VeryHigh);
    let cs = Output::new(cs_pin, Level::High, Speed::VeryHigh);
    let dc = Output::new(dc_pin, Level::Low, Speed::VeryHigh);

    let dev = SpiDevice::new(spi_bus, cs);
    let mut delay = embassy_time::Delay;

    let interface = mipidsi::interface::SpiInterface::new(dev, dc, buffer);
    mipidsi::Builder::new(ILI9341Rgb565, interface)
        .reset_pin(reset)
        .init(&mut delay)
        .unwrap()
}
