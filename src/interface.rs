use macroquad::prelude::*;

use crate::chip8::{Chip8, DISPLAY_HEIGHT, DISPLAY_WIDTH};

pub struct Interface {
    pixel_scaling: u16,
}

impl Interface {
    pub fn new() -> Interface {
        Interface { pixel_scaling: 10 }
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

                draw_rectangle(
                    (x as u16 * self.pixel_scaling) as f32,
                    (y as u16 * self.pixel_scaling) as f32,
                    self.pixel_scaling as f32,
                    self.pixel_scaling as f32,
                    color,
                )
            }
        }
    }
}
