use crate::PI;
use core::f32::INFINITY;
use embedded_graphics::{
    pixelcolor::Rgb565,
    prelude::{RgbColor, WebColors},
};
use micromath::F32Ext;

const MAP_COLORS_H: [Rgb565; 3] = [Rgb565::BLACK, Rgb565::CSS_RED, Rgb565::CSS_LAVENDER];
const MAP_COLORS_V: [Rgb565; 3] = [Rgb565::BLACK, Rgb565::CSS_DARK_RED, Rgb565::CSS_DARK_CYAN];

pub fn cast_ray(x: f32, y: f32, theta: f32) -> (f32, Rgb565) {
    // Check Horizontal lines
    let (hx, hy, colorh);

    'horizontal: {
        let theta = (theta + 2.0 * PI) % (2.0 * PI);
        let atan = F32Ext::atan(theta);

        let (mut rx, mut ry, mut dof, dy, dx);
        if theta == PI || theta == 0.0 {
            hx = INFINITY;
            hy = INFINITY;
            colorh = MAP_COLORS_H[0];
            break 'horizontal;
        } else if theta > PI {
            // Looking up
            ry = F32Ext::floor(y);
            rx = x - (y - ry) * atan;
            dy = -1.0;
            dx = dy * atan;
            dof = 0;
            //println!("UP: ø={}, ry={}, rx={}", theta, ry, rx)
        } else {
            ry = F32Ext::floor(y + 1.0);
            rx = x - (y - ry) * atan;
            dy = 1.0;
            dx = dy * atan;
            dof = 0;
            //println!("DOWN: ø={}, ry={}, rx={}", theta, ry, rx)
        }
        let mut colorindex = 0;
        while dof < 8 {
            if crate::game::map::inside_map(rx, ry) {
                let y_index = if dy <= 0.0 {
                    ry as usize - 1
                } else {
                    ry as usize
                };

                if crate::game::MAP[y_index][rx as usize] != 0 {
                    colorindex = crate::game::MAP[y_index][rx as usize];
                    break;
                } else {
                    ry += dy;
                    rx += dx;
                }
                dof += 1;
            } else {
                break;
            }
        }
        colorh = MAP_COLORS_H[colorindex as usize];
        hx = rx;
        hy = ry;
    }
    let (vx, vy, colorv);

    'vertical: {
        let ntan = -F32Ext::tan(theta);

        let (mut rx, mut ry, mut dof, dy, dx);

        if theta == PI / 2.0 || theta == 3.0 * PI / 2.0 {
            vx = INFINITY;
            vy = INFINITY;
            colorv = MAP_COLORS_H[0];
            break 'vertical;
        } else if 3.0 * PI / 2.0 <= theta || theta < PI / 2.0 {
            // Looking right
            rx = F32Ext::floor(x + 1.0);
            ry = y + (x - rx) * ntan;
            dx = 1.0;
            dy = -dx * ntan;
            dof = 0;
        } else {
            rx = F32Ext::floor(x);
            ry = y + (x - rx) * ntan;
            dx = -1.0;
            dy = -dx * ntan;
            dof = 0;
        }
        let mut colorindex = 0;
        while dof < 8 {
            if crate::game::map::inside_map(rx, ry) {
                let x_index = if dx <= 0.0 {
                    rx as isize - 1
                } else {
                    rx as isize
                };

                if crate::game::MAP[ry as usize][x_index as usize] != 0 {
                    colorindex = crate::game::MAP[ry as usize][x_index as usize];
                    break;
                } else {
                    ry += dy;
                    rx += dx;
                }
                dof += 1;
            } else {
                break;
            }
        }
        vx = rx;
        vy = ry;
        colorv = MAP_COLORS_V[colorindex as usize];
    }
    let dh = F32Ext::sqrt((hx - x) * (hx - x) + (hy - y) * (hy - y));
    let dv = F32Ext::sqrt((vx - x) * (vx - x) + (vy - y) * (vy - y));

    if dh < dv {
        return (dh, colorh);
    } else {
        return (dv, colorv);
    }
}
