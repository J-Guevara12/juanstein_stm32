use embedded_graphics::{
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{PrimitiveStyleBuilder, Rectangle},
};

use crate::rendering::raycaster::cast_ray;
use crate::PI;

use crate::display::DisplayType;
use libm::cosf;

use super::map;

const PLAYER_SIZE: u32 = 3;
const PLAYER_SPEED: f32 = 4.0;
const ANGULAR_SPEED: f32 = 0.005;
const PLAYER_COLOR: Rgb565 = Rgb565::CSS_AQUA;
use crate::display::BUFFER_COLUMN_SIZE;

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
        let mut counter = 0;
        for i in 0..n_steps {
            counter += 1;
            let dt = i as f32 * max_angle / n_steps as f32 - max_angle / 2.0;
            let (dist, color) = cast_ray(self.px, self.py, self.theta + dt);
            let height = crate::HEIGHT as f32 / (dist * cosf(dt) * 2.5);
            let y0 = ((crate::HEIGHT as f32 - height) / 2.0) as i32;
            let rect = Rectangle::new(Point::new(y0, i as i32), Size::new(height as u32, 1));
            if height as usize <= crate::HEIGHT {
                let top = Rectangle::new(
                    Point::new(0, i as i32),
                    Size::new((crate::HEIGHT - y0 as usize - height as usize) as u32, 1),
                );
                let floor = Rectangle::new(
                    Point::new(y0 + height as i32, i as i32),
                    Size::new((crate::HEIGHT - y0 as usize - height as usize) as u32, 1),
                );
                display.fill_solid(&floor, Rgb565::CSS_LIME).await.unwrap();
                display.flush().await.unwrap();
                display
                    .fill_solid(&top, Rgb565::CSS_SKY_BLUE)
                    .await
                    .unwrap();
                display.flush().await.unwrap();
            }
            display.fill_solid(&rect, color).await.unwrap();
            display.flush().await.unwrap();
        }
    }

    fn move_player(&mut self, displacement: f32, theta: f32) {
        self.px += displacement * libm::cosf(theta);
        self.py += displacement * libm::sinf(theta);
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
