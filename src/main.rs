use chip8::Chip8;
use interface::Interface;
use macroquad::prelude::*;

mod chip8;
mod interface;

#[macroquad::main("Chip8")]
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
