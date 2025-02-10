use embedded_graphics::{
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{PrimitiveStyleBuilder, Rectangle},
};

use crate::PI;
use crate::{rendering::raycaster::cast_ray, HEIGHT};

use crate::display::DisplayType;
use crate::display::BUFFER_COLUMN_SIZE;

use super::map;
use micromath::F32Ext;

const PLAYER_SIZE: u32 = 3;
const PLAYER_SPEED: f32 = 4.0;
const ANGULAR_SPEED: f32 = 0.005;
const PLAYER_COLOR: Rgb565 = Rgb565::CSS_AQUA;

pub struct Player {
    px: f32,
    py: f32,
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

    pub fn draw<D>(&self, display: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Rgb565>,
    {
        let x = self.px * map::GRID_SIZE as f32;
        let y = self.py * map::GRID_SIZE as f32;

        Rectangle::new(
            Point::new(
                (x as u32 - PLAYER_SIZE / 2) as i32,
                (y as u32 - PLAYER_SIZE / 2) as i32,
            ),
            Size::new(PLAYER_SIZE, PLAYER_SIZE),
        )
        .into_styled(
            PrimitiveStyleBuilder::new()
                .fill_color(PLAYER_COLOR)
                .build(),
        )
        .draw(display)?;

        Ok(())
    }

    pub async fn draw_player_raycaster<'a>(&self, display: &mut DisplayType<'a>) {
        let n_steps = crate::WIDTH;

        let max_angle = PI / 2.0;
        let _area = Rectangle::new(Point::new(0, 0), Size::new(240, BUFFER_COLUMN_SIZE as u32));
        for i in 0..n_steps {
            let mut _counter = 0;
            if i % BUFFER_COLUMN_SIZE == 0 {
                display.flush().await.unwrap();
            }

            let dt = i as f32 * max_angle / n_steps as f32 - max_angle / 2.0;

            let (dist, color) = cast_ray(self.px, self.py, self.theta + dt);

            let height =
                F32Ext::round(crate::HEIGHT as f32 / (dist * F32Ext::cos(dt) * 2.5)) as usize;
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
                    .fill_solid_in_context(&ceiling, Rgb565::CSS_SKY_BLUE)
                    .unwrap();
                display
                    .fill_solid_in_context(&floor, Rgb565::CSS_LAWN_GREEN)
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

    fn move_player(&mut self, displacement: f32, theta: f32) {
        self.px += displacement * F32Ext::cos(theta);
        self.py += displacement * F32Ext::sin(theta);
    }

    fn update_angle(&mut self, increment: f32) {
        self.theta = (self.theta + increment) % (2.0 * PI);
        if self.theta < 0.0 {
            self.theta += 2.0 * PI;
        }
    }
    /*
        pub fn update_pos(&mut self, time_moving: u128) {
            let displacement = time_moving as f32 * PLAYER_SPEED / 1000.0;
            match keycode {
                Keycode::W => self.move_player(displacement, self.theta),
                Keycode::A => self.move_player(displacement, self.theta - PI / 2.0),
                Keycode::S => self.move_player(displacement, self.theta + PI),
                Keycode::D => self.move_player(displacement, self.theta + PI / 2.0),
                Keycode::L => self.update_angle(time_moving as f32 * ANGULAR_SPEED),
                Keycode::J => self.update_angle(-1.0 * time_moving as f32 * ANGULAR_SPEED),
                _ => (),
            }
        }
    */
}
