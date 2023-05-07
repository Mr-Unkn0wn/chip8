use egui::*;
use macroquad::prelude::*;

use crate::chip8::Chip8;

pub struct Interface {
    pixel_scaling: u16,
    pub debug: bool,
    pub manual_step: bool,
}

impl Interface {
    pub fn new() -> Interface {
        Interface {
            pixel_scaling: 10,
            debug: true,
            manual_step: false,
        }
    }

    pub fn draw_gui(&mut self, chip8: &Chip8) {
        clear_background(DARKGRAY);

        self.draw_chip_display(chip8);

        self.draw_interface(chip8);
        egui_macroquad::draw();
    }

    fn draw_chip_display(&mut self, chip8: &Chip8) {
        let display = chip8.display();

        for (x, col) in display.iter().enumerate() {
            for (y, value) in col.iter().enumerate() {
                let color = match value {
                    true => WHITE,
                    false => BLACK,
                };

                draw_rectangle(
                    (100 + x * self.pixel_scaling as usize) as f32,
                    (100 + y * self.pixel_scaling as usize) as f32,
                    self.pixel_scaling as f32,
                    self.pixel_scaling as f32,
                    color,
                )
            }
        }
    }

    fn draw_interface(&mut self, chip8: &Chip8) {
        egui_macroquad::ui(|ctx| {
            egui::TopBottomPanel::top("Topbar").show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.add(egui::Button::new("Load"));
                    ui.add(egui::Button::new("Reset"));
                    ui.separator();
                    ui.add(egui::Button::new("Settings"));
                    ui.separator();
                    if ui.add(egui::Button::new("Debug")).clicked() {
                        self.debug = !self.debug;
                    }
                })
            });

            if self.debug {
                self.draw_debug_menu(ctx, chip8)
            }
        });
    }

    fn draw_debug_menu(&mut self, ctx: &Context, chip8: &Chip8) {
        egui::SidePanel::right("My Window")
            .min_width(200.0)
            .show(ctx, |ui| {
                self.draw_register_grid(ui, chip8);
                ui.separator();
                if ui.add(egui::Button::new("Next")).clicked() {
                    self.manual_step = true;
                }
            });
    }

    fn draw_register_grid(&mut self, ui: &mut Ui, chip8: &Chip8) {
        egui::Grid::new("RegisterGrid").show(ui, |ui| {
            for i in 0..=14 {
                if i != 0 && i % 3 == 0 {
                    ui.end_row();
                }
                ui.label(format!("R{:X} {:X}", i, chip8.register()[i]));
            }
            ui.end_row();
            ui.label(format!("R{:X} {:X}", 15, chip8.register()[15]));
            ui.label(format!("DT {:X}", chip8.delay_timer()));
            ui.label(format!("ST {:X}", chip8.sound_timer()));
            ui.end_row();
            ui.label(format!("I {:X}", chip8.index()));
            ui.separator();
            ui.label(format!("PC {:X}", chip8.program_counter()));
        });
    }
}
