use eframe::{NativeOptions, egui};

struct MyApp {
    label: String,
    value: i32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            label: "My_photo_editor".to_owned(),
            value: 0,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My_photo_editor");
            ui.label(&self.label);
            ui.add(egui::Slider::new(&mut self.value, 0..=100).text("Contrast"));
            ui.label(format!("Actual value : {}", self.value));
            if ui.button("Click me !").clicked() {
                self.value += 1;
            }
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
