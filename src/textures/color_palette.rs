use embedded_graphics::{
    pixelcolor::Rgb565,
    prelude::{RgbColor, WebColors},
};

const PALETTE_LEN: usize = 4;

pub static COLOR_PALETTE_V: [Rgb565; PALETTE_LEN] = [
    Rgb565::CSS_FIRE_BRICK,
    Rgb565::CSS_SLATE_GRAY,
    Rgb565::CSS_BLACK,
    Rgb565::YELLOW,
];

pub static COLOR_PALETTE_H: [Rgb565; PALETTE_LEN] = [
    Rgb565::new(16, 5, 2),
    Rgb565::new(9, 23, 11),
    Rgb565::CSS_BLACK,
    Rgb565::new(26, 26, 0),
];
