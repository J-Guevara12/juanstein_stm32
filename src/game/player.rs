use core::usize;

use crate::{
    display::{DisplayType, BUFFER_COLUMN_SIZE},
    rendering::raycaster::cast_ray,
    textures::{COLOR_PALETTE_H, COLOR_PALETTE_V, TEXTURES, TEXTURE_SIZE},
    HEIGHT, PI,
};

use embedded_graphics::{pixelcolor::Rgb565, prelude::*, primitives::Rectangle};

use super::MAP;
use micromath::F32Ext;

// Determines the height of the walls (recommended between 1.0 and 4.0)
const SCALING_FACTOR: f32 = 1.2;
// How quick the player will move
const PLAYER_SPEED: f32 = 3.5;
// Gyroscope sensibility
const PLAYER_ANGULAR_SPEED: f32 = 0.10;

#[derive(Clone)]
pub struct Player {
    pub px: f32,    // Position at x
    pub py: f32,    // Position at y
    pub theta: f32, // Angle between camera and x axis
}

impl Player {
    /// Creates a new player object.
    ///
    /// # Arguments
    ///
    /// * `x` - Initial position at x
    /// * `y` - Initial position at y
    ///
    /// # Examples
    ///
    /// Creates a new player at the center of the tile (1,1)
    /// ```
    /// let mut player = Player::new(1.5, 1.5, PI/2.0);
    /// ```
    pub fn new(x: f32, y: f32) -> Player {
        return Player {
            px: x,
            py: y,
            theta: 2.0 * PI - PI / 2.0,
        };
    }

    /// Default player at the center of the tile(1,1) and looking at the x axis
    ///
    /// # Examples
    ///
    /// ```
    /// let mut player = Player::default();
    /// ```
    pub const fn default() -> Player {
        return Player {
            px: 1.5,
            py: 1.5,
            theta: 0.0,
        };
    }

    /// Draws the player POV using raycasting
    ///
    /// # Arguments
    ///
    /// * `display` - Display Object where the camera will be displayed.
    ///
    /// # Examples
    ///
    /// ```
    /// player.draw_player_raycaster(&mut display);
    /// ```
    pub async fn draw_player_raycaster<'a>(&self, display: &mut DisplayType<'a>) {
        let n_steps = crate::WIDTH;

        let max_angle = PI / 3.0;

        let _area = Rectangle::new(Point::new(0, 0), Size::new(240, BUFFER_COLUMN_SIZE as u32));
        for i in 0..n_steps {
            if i % BUFFER_COLUMN_SIZE == 0 {
                display.flush().await.unwrap();
            }

            let dt = i as f32 * max_angle / n_steps as f32 - max_angle / 2.0;

            let (dist, is_vertical, texture_index, column_index) =
                cast_ray(self.px, self.py, self.theta + dt);

            let height =
                F32Ext::round(crate::HEIGHT as f32 / (dist * F32Ext::cos(dt) * SCALING_FACTOR))
                    as usize;
            let y0 = (crate::HEIGHT - height) as i32 / 2;

            if height < HEIGHT {
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

            // Iterates over the texture
            for y in 0..TEXTURE_SIZE {
                let y_0 = y0 + (y * height / TEXTURE_SIZE) as i32;
                let h = (y + 1) * height / TEXTURE_SIZE - y * height / TEXTURE_SIZE;
                let h = if (y_0 + h as i32) < 0 { 0 } else { h };

                let rect = Rectangle::new(
                    Point::new(y_0, (i % BUFFER_COLUMN_SIZE as usize) as i32),
                    Size::new(h as u32, 1),
                );
                let idx = TEXTURES[texture_index][column_index][y] as usize;
                let color = if is_vertical {
                    COLOR_PALETTE_V[idx]
                } else {
                    COLOR_PALETTE_H[idx]
                };
                // TODO: DMA Fill (memory to memory transfer)
                display.fill_solid_in_context(&rect, color).unwrap();
            }

            // Updates the context to write the new screen segment.
            if i % BUFFER_COLUMN_SIZE == 0 {
                let area = Rectangle::new(
                    Point::new(0, i as i32),
                    Size::new(240, BUFFER_COLUMN_SIZE as u32),
                );
                display.set_context(&area).await.unwrap();
            }
        }
    }

    /// Moves the player according to the joystick input (2 degrees of freedom)
    ///
    /// # Arguments
    ///
    /// * `x` - X input in the joystick.
    /// * `y` - Y Input in the joystick.
    ///
    /// # Examples
    ///
    /// ```
    /// // Move the player forward
    /// player._move(1.0, 0.0)
    /// // Move the player backwards
    /// player._move(-1.0, 0.0)
    /// // Move the player to the left
    /// player._move(0.0, -1.0)
    /// // Move the player to the right
    /// player._move(0.0, 1.0)
    /// ```
    pub fn _move(&mut self, x: f32, y: f32) {
        let dx =
            (x * F32Ext::cos(self.theta) + y * F32Ext::cos(self.theta + PI / 2.0)) * PLAYER_SPEED;
        let dy =
            (x * F32Ext::sin(self.theta) + y * F32Ext::sin(self.theta + PI / 2.0)) * PLAYER_SPEED;

        let posx = F32Ext::floor(self.px) as usize;
        let posy = F32Ext::floor(self.py) as usize;

        // Colition detection in x
        {
            let posx = F32Ext::floor(self.px + dx) as usize;
            if MAP[posy][posx] == 0 || MAP[posy][posx] == 4 {
                self.px += dx;
            }
        }
        // Colition detection in y
        {
            let posy = F32Ext::floor(self.py + dy) as usize;
            if MAP[posy][posx] == 0 || MAP[posy][posx] == 4 {
                self.py += dy;
            }
        }
    }

    /// Rotate the player according to the gyroscope input.
    ///
    /// # Arguments
    ///
    /// * `increment` - How much the player will be rotated.
    ///
    /// # Examples
    ///
    /// ```
    /// // Rotates to the right
    /// player.rotate_player(PI/2).
    /// // Rotates to the left
    /// player.rotate_player(-PI/2).
    /// ```
    pub fn rotate_player(&mut self, increment: f32) {
        // Value always will be bound between 0.0 and 2*PI.
        self.theta = (self.theta + increment * PLAYER_ANGULAR_SPEED) % (2.0 * PI);
        if self.theta < 0.0 {
            self.theta += 2.0 * PI;
        }
    }
}
