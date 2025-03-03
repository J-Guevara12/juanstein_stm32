mod all_black;
mod color_palette;
mod door_1;
mod wall_1;
mod wall_2;
mod wall_3;

pub const TEXTURE_SIZE: usize = 20;
pub use color_palette::COLOR_PALETTE_H;
pub use color_palette::COLOR_PALETTE_V;

pub const TEXTURES: [[[u8; TEXTURE_SIZE]; TEXTURE_SIZE]; 5] = [
    all_black::TEXTURE,
    wall_1::TEXTURE,
    wall_2::TEXTURE,
    wall_3::TEXTURE,
    door_1::TEXTURE,
];
