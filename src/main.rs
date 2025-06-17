use std::io::Read;

use eframe::{NativeOptions, egui};

struct MyApp {
    file: Option<String>,
    contrast: f32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            file: None,
            contrast: 0.0,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui_extras::install_image_loaders(ctx);
            ctx.set_pixels_per_point(1.2f32);
            ui.heading("My_photo_editor");
            ui.label("Contrast");
            ui.add(egui::Slider::new(&mut self.contrast, -10f32..=10f32));
            ui.horizontal(|ui| {
                let button_decrease = ui.button("➖");
                let button_reset = ui.button("Reset");
                let button_increase = ui.button("➕");
                if button_reset.clicked() {
                    self.contrast = 0.0;
                }
                if button_decrease.clicked() {
                    self.contrast -= 0.1;
                }
                if button_increase.clicked() {
                    self.contrast += 0.1;
                }
            });
            ui.image(self.file.clone().unwrap());
        });
    }
}

fn main() {
    let native_options = NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size(egui::vec2(320.0, 240.0))
            .with_min_inner_size(egui::vec2(300.0, 200.0)),
        ..Default::default()
    };
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: my_photo_editor <image_file>");
        std::process::exit(1);
    }
    let app = MyApp {
        file: Some(args[1].clone()),
        ..Default::default()
    };
    eframe::run_native(
        "My_photo_editor",
        native_options,
        Box::new(|_cc| Ok(Box::new(app))),
    )
    .expect("Unable to open a egui window !");
}
