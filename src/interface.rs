use macroquad::prelude::*;

use crate::chip8::{Chip8, DISPLAY_HEIGHT, DISPLAY_WIDTH};

pub struct Interface {
    image: Image,
    texture: Texture2D,
    texture_params: DrawTextureParams,
}

impl Interface {
    pub fn new() -> Interface {
        let image = Image::gen_image_color(DISPLAY_WIDTH as u16, DISPLAY_HEIGHT as u16, RED);
        let texture = Texture2D::from_image(&image);
        texture.set_filter(FilterMode::Linear);
        let texture_params = DrawTextureParams {
            dest_size: Some(Vec2::new(screen_width(), screen_height())),
            source: None,
            rotation: 0.0,
            flip_x: false,
            flip_y: false,
            pivot: None,
        };

        Interface {
            image,
            texture,
            texture_params,
        }
    }

    pub fn draw_gui(&mut self, chip8: &Chip8) {
        clear_background(GREEN);

        self.draw_chip_display(chip8);
    }

    fn draw_chip_display(&mut self, chip8: &Chip8) {
        let display = chip8.display();

        for y in 0..DISPLAY_HEIGHT {
            for x in 0..DISPLAY_WIDTH {
                let white = display[y * DISPLAY_HEIGHT + x];

                let color = match white {
                    true => WHITE,
                    false => BLACK,
                };

                self.image.set_pixel(x as u32, y as u32, color)
            }
        }

        self.texture.update(&self.image);
        draw_texture_ex(self.texture, 0.0, 0.0, WHITE, self.texture_params.clone())
    }
}
