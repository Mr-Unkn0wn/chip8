use chip8::{Chip8, DISPLAY_HEIGHT, DISPLAY_WIDTH};
use interface::Interface;
use macroquad::prelude::*;

mod chip8;
mod interface;

fn window_conf() -> Conf {
    Conf {
        window_title: "Chip8".to_string(),
        fullscreen: false,
        window_width: (DISPLAY_WIDTH * 10) as i32,
        window_height: (DISPLAY_HEIGHT * 10) as i32,
        ..Default::default()
    }
}

#[macroquad::main(window_conf())]
async fn main() {
    let rom = include_bytes!("E:\\Code\\Rust\\chip8\\roms\\IBM Logo.ch8");

    let mut chip8 = Chip8::new(rom);
    let mut interface = Interface::new();

    loop {
        chip8.step();
        interface.draw_gui(&chip8);
        next_frame().await
    }
}
