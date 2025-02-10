use embassy_stm32::rcc::*;
use embassy_stm32::time::Hertz;
use embassy_stm32::Config;

pub fn configure_rcc(config: &mut Config) {
    config.rcc.sys = Sysclk::PLL1_R;
    config.rcc.hse = Some(Hse {
        freq: Hertz::mhz(8),
        mode: HseMode::Oscillator,
    });
    config.rcc.pll = Some(Pll {
        source: PllSource::HSE,
        prediv: PllPreDiv::DIV1,
        mul: PllMul::MUL20,
        divp: None,
        divq: None,
        divr: Some(PllRDiv::DIV2),
    });
}
