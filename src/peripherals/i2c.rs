use embassy_stm32::{
    bind_interrupts,
    i2c::{ErrorInterruptHandler, EventInterruptHandler},
    interrupt::typelevel::Binding,
    mode::Async,
    peripherals,
    time::Hertz,
};

bind_interrupts!(struct Irqs {
    I2C1_EV => embassy_stm32::i2c::EventInterruptHandler<peripherals::I2C1>;
    I2C1_ER => embassy_stm32::i2c::ErrorInterruptHandler<peripherals::I2C1>;
});

const I2C_FREQ: u32 = 100_000;

pub fn init<T, U, V, W, X>(
    i2c: T,
    scl: U,
    sda: V,
    dma_tx: W,
    dma_rx: X,
) -> embassy_stm32::i2c::I2c<'static, Async>
where
    T: embassy_stm32::i2c::Instance,
    U: embassy_stm32::i2c::SclPin<T>,
    V: embassy_stm32::i2c::SdaPin<T>,
    W: embassy_stm32::i2c::TxDma<T>,
    X: embassy_stm32::i2c::RxDma<T>,
    Irqs: Binding<<T as embassy_stm32::i2c::Instance>::EventInterrupt, EventInterruptHandler<T>>
        + Binding<<T as embassy_stm32::i2c::Instance>::ErrorInterrupt, ErrorInterruptHandler<T>>,
{
    embassy_stm32::i2c::I2c::new(
        i2c,
        scl,
        sda,
        Irqs,
        dma_tx,
        dma_rx,
        Hertz(I2C_FREQ),
        Default::default(),
    )
}
