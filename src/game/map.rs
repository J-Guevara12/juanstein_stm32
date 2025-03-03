pub const MAP: [[u8; 16]; 16] = [
    [1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    [1, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1],
    [1, 1, 2, 1, 3, 3, 1, 0, 0, 0, 0, 0, 0, 1, 0, 1],
    [1, 0, 0, 0, 0, 2, 1, 0, 0, 0, 0, 0, 0, 3, 0, 1],
    [1, 0, 3, 3, 3, 3, 1, 0, 0, 0, 0, 0, 0, 4, 0, 1],
    [1, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 3, 3, 1],
    [1, 0, 3, 3, 3, 3, 1, 1, 1, 1, 1, 1, 1, 1, 2, 1],
    [1, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 1],
    [1, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 1],
    [1, 0, 2, 0, 0, 0, 0, 0, 2, 3, 0, 0, 0, 3, 0, 1],
    [1, 0, 3, 0, 0, 0, 0, 0, 3, 2, 0, 0, 0, 3, 0, 1],
    [1, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 1],
    [1, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 1],
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
];

pub fn inside_map(x: f32, y: f32) -> bool {
    let x = x as isize;
    let y = y as isize;

    let inside_y = 0 < y && y < MAP.len() as isize;
    let inside_x = 0 < x && x < MAP.len() as isize;

    return inside_y && inside_x;
}
