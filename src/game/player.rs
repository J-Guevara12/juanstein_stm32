use embedded_graphics::{
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{PrimitiveStyleBuilder, Rectangle},
};

use crate::PI;
use crate::{rendering::raycaster::cast_ray, HEIGHT};

use crate::display::DisplayType;
use crate::display::BUFFER_COLUMN_SIZE;
use micromath::F32Ext;

use super::MAP;

const PLAYER_SIZE: u32 = 3;
const PLAYER_SPEED: f32 = 2.5;
const PLAYER_ANGULAR_SPEED: f32 = 0.05;

#[derive(Clone)]
pub struct Player {
    pub px: f32,
    pub py: f32,
    pub theta: f32,
}

impl Player {
    pub fn new(x: f32, y: f32) -> Player {
        return Player {
            px: x,
            py: y,
            theta: 2.0 * PI - PI / 2.0,
        };
    }

    pub const fn default() -> Player {
        return Player {
            px: 1.5,
            py: 1.5,
            theta: 0.0,
        };
    }

    pub async fn draw_player_raycaster<'a>(&self, display: &mut DisplayType<'a>) {
        let n_steps = crate::WIDTH;

        let max_angle = PI / 3.0;

        let _area = Rectangle::new(Point::new(0, 0), Size::new(240, BUFFER_COLUMN_SIZE as u32));
        for i in 0..n_steps {
            if i % BUFFER_COLUMN_SIZE == 0 {
                display.flush().await.unwrap();
            }

            let dt = i as f32 * max_angle / n_steps as f32 - max_angle / 2.0;

            let (dist, color) = cast_ray(self.px, self.py, self.theta + dt);

            let height =
                F32Ext::round(crate::HEIGHT as f32 / (dist * F32Ext::cos(dt) * 3.5)) as usize;
            let height = usize::min(crate::HEIGHT, height);
            let y0 = (crate::HEIGHT - height) as i32 / 2;

            let rect = Rectangle::new(
                Point::new(y0, (i % BUFFER_COLUMN_SIZE as usize) as i32),
                Size::new(height as u32, 1),
            );

            if height != HEIGHT {
                let ceiling = Rectangle::new(
                    Point::new(0, (i % BUFFER_COLUMN_SIZE as usize) as i32),
                    Size::new((HEIGHT - height) as u32 / 2, 1),
                );
                let floor = Rectangle::new(
                    Point::new(
                        (HEIGHT + height) as i32 / 2,
                        (i % BUFFER_COLUMN_SIZE as usize) as i32,
                    ),
                    Size::new((HEIGHT as u32 - height as u32) / 2, 1),
                );
                display
                    .fill_solid_in_context(&ceiling, Rgb565::CSS_DEEP_SKY_BLUE)
                    .unwrap();
                display
                    .fill_solid_in_context(&floor, Rgb565::CSS_FOREST_GREEN)
                    .unwrap();
            }

            // TODO: DMA Fill (memory to memory transfer)
            display.fill_solid_in_context(&rect, color).unwrap();

            if i % BUFFER_COLUMN_SIZE == 0 {
                let area = Rectangle::new(
                    Point::new(0, i as i32),
                    Size::new(240, BUFFER_COLUMN_SIZE as u32),
                );
                display.set_context(&area).await.unwrap();
            }
        }
    }

    pub fn move_player(&mut self, x: f32, y: f32) {
        let dx = x * F32Ext::cos(self.theta) * PLAYER_SPEED + y * F32Ext::sin(self.theta);
        let dy = y * F32Ext::cos(self.theta) * PLAYER_SPEED + x * F32Ext::cos(self.theta);

        let posx = F32Ext::floor(self.px) as usize;
        let posy = F32Ext::floor(self.py) as usize;

        {
            let posx = F32Ext::floor(self.px + dx) as usize;
            if MAP[posy][posx] == 0 {
                self.px += dx;
            }
        }
        {
            let posy = F32Ext::floor(self.py + dy) as usize;
            if MAP[posy][posx] == 0 {
                self.py += dy;
            }
        }
    }

    pub fn rotate_player(&mut self, increment: f32) {
        self.theta = (self.theta + increment * PLAYER_ANGULAR_SPEED) % (2.0 * PI);
        if self.theta < 0.0 {
            self.theta += 2.0 * PI;
        }
    }
}
