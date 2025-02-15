use embassy_stm32::adc::{Adc, Instance};
pub fn init<ADC>(adc: ADC) -> Adc<'static, ADC>
where
    ADC: Instance,
{
    let adc = Adc::new(adc);

    adc
}
