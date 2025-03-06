# Peripheral configuration:

## RCC Configuration

| Parameter  | Value      |
|------------|------------|
| SYS        | PLL1       |
| HSE_FREQ   | 8MHZ       |
| HSE_MODE   | Oscillator |
| ADCSEL     | SYS        |
| PLL_SOURCE | HSE        |
| PLL_PREDIV | 1          |
| PLL_MUL    | 20         |
| PLL_DIVR   | 2          |
| Final Freq | 80 MHz     |

## i²C1

| Parameter   | Value    |
|-------------|----------|
| SDA Pull-up | False    |
| SCL Pull-up | False    |
| Timeout     | 1000 ms  |
| SCL         | PB8      |
| SDA         | PB9      |
| TX_DMA      | DMA1 CH6 |
| RX_DMA      | DMA1 CH7 |


## ADC1

| Parameter     | Value  |
|---------------|--------|
| Sampling Rate | 40 Hz  |
| Resolution    | 6 bits |
| Channel X     | PC0    |
| Channel Y     | PC1    |

## Display

| Parameter | Value |
|-----------|-------|
| Reset     | PC3   |
| CS        | PB0   |
| DC        | PC2   |
| Interface | SPI1  |

## SPI1 (TX only)

| Parameter | Value                       |
|-----------|-----------------------------|
| Frequency | 40 MHz                      |
| Polarity  | Idle Low                    |
| Phase     | Capture on first transition |
| SCK       | PA5                         |
| MOSI      | PA7                         |
| TX_DMA    | DMA1 CH3                    |

