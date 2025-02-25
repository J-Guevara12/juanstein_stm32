use embedded_graphics::{
    pixelcolor::Rgb666,
    prelude::*,
    primitives::{PrimitiveStyle, PrimitiveStyleBuilder, Rectangle},
};

use crate::{HEIGHT, WIDTH};

pub const MAP: [[u8; 8]; 8] = [
    [1, 1, 1, 1, 1, 1, 1, 1],
    [1, 0, 0, 1, 0, 0, 1, 1],
    [2, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 2, 0, 1, 1, 0, 1],
    [1, 0, 0, 0, 1, 0, 0, 1],
    [2, 0, 1, 0, 0, 0, 0, 1],
    [1, 0, 1, 0, 0, 1, 0, 1],
    [1, 1, 1, 1, 1, 1, 1, 1],
];

const fn grid_size() -> i32 {
    let l1 = (WIDTH as usize / MAP.len()) as i32;
    let l2 = (HEIGHT as usize / MAP[0].len()) as i32;
    if l1 < l2 {
        l1
    } else {
        l2
    }
}

const BACKGROUND_COLOR: Rgb666 = Rgb666::WHITE;
const LINE_COLOR: Rgb666 = Rgb666::BLACK;

const SPACE_STYLE: PrimitiveStyle<Rgb666> = PrimitiveStyleBuilder::new()
    .stroke_color(LINE_COLOR)
    .stroke_width(1)
    .fill_color(BACKGROUND_COLOR)
    .build();

const WALL_STYLE: PrimitiveStyle<Rgb666> = PrimitiveStyleBuilder::new()
    .stroke_color(LINE_COLOR)
    .stroke_width(1)
    .fill_color(LINE_COLOR)
    .build();

static STYLES: [PrimitiveStyle<Rgb666>; 2] = [SPACE_STYLE, WALL_STYLE];

pub const GRID_SIZE: i32 = grid_size();

pub fn inside_map(x: f32, y: f32) -> bool {
    let x = x as isize;
    let y = y as isize;

    let inside_y = 0 < y && y < MAP.len() as isize;
    let inside_x = 0 < x && x < MAP.len() as isize;

    return inside_y && inside_x;
}

pub fn draw_map<D>(display: &mut D) -> Result<(), D::Error>
where
    D: DrawTarget<Color = Rgb666>,
{
    for (y, row) in MAP.iter().enumerate() {
        for (x, value) in row.iter().enumerate() {
            Rectangle::new(
                Point::new(x as i32 * GRID_SIZE, y as i32 * GRID_SIZE),
                Size::new(GRID_SIZE as u32 + 1, GRID_SIZE as u32 + 1),
            )
            .into_styled(STYLES[*value as usize])
            .draw(display)?
        }
    }
    Ok(())
}
