#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use egui_extras::RetainedImage;
use rand::Rng;
use std::{fs, io};

fn main() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(500.0, 900.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Show an image with eframe/egui",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}

struct MyApp {
    image: RetainedImage,
}

impl Default for MyApp {
    fn default() -> Self {
        // Select random image from directory
        let image_directory = "./src/mary";
        let paths: Vec<Result<fs::DirEntry, io::Error>> =
            fs::read_dir(image_directory).unwrap().collect();
        let index = rand::thread_rng().gen_range(0..paths.len()-1);
        let image_name = if let Some(image) = paths.get(index) {
            image.as_ref().unwrap().file_name().to_str().unwrap().to_string()
        } else {
            unimplemented!()
        };

        let image_bytes = image_name.as_bytes();
        Self {
            image: RetainedImage::from_image_bytes(image_name.clone(), image_bytes)
                .unwrap(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("This is an image:");
            self.image.show(ui);

            ui.heading("This is a rotated image:");
            ui.add(
                egui::Image::new(self.image.texture_id(ctx), self.image.size_vec2())
                    .rotate(45.0_f32.to_radians(), egui::Vec2::splat(0.5)),
            );

            ui.heading("This is an image you can click:");
            ui.add(egui::ImageButton::new(
                self.image.texture_id(ctx),
                self.image.size_vec2(),
            ));
        });
    }
}
