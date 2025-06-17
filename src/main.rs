use eframe::{NativeOptions, egui};

struct MyApp {
    contrast: f32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self { contrast: 0.0 }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
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
            })
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
    eframe::run_native(
        "My_photo_editor",
        native_options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
    .expect("Unable to open a egui window !");
}
