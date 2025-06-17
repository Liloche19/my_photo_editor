use eframe::{NativeOptions, egui};
use image::io::Reader as ImageReader;
use std::path::PathBuf;

struct MyApp {
    image_path: Option<String>,
    image_texture: Option<egui::TextureHandle>,
    contrast: f32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            image_path: None,
            image_texture: None,
            contrast: 0.0,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.load_image(ctx);
            egui_extras::install_image_loaders(ctx);
            ctx.set_pixels_per_point(1.2f32);
            ui.heading("My_photo_editor");
            if let Some(texture) = &self.image_texture {
                let [width, height] = texture.size();
                let image_size = egui::vec2(width as f32, height as f32);
                let image_widget = egui::Image::new((texture.id(), image_size));
                ui.add(image_widget);
            } else if self.image_path.is_some() {
                ui.label("Chargement de l'image...");
            }
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
        });
    }
}

impl MyApp {
    fn load_image(&mut self, ctx: &egui::Context) {
        self.image_texture = None;
        let image_path = match &self.image_path {
            Some(s) => s,
            None => std::process::exit(1),
        };
        let path = PathBuf::from(image_path);
        if !path.exists() {
            std::process::exit(1);
        }
        match ImageReader::open(&path) {
            Ok(reader) => match reader.decode() {
                Ok(dynamic_image) => {
                    let image_rgba = dynamic_image.to_rgba8();
                    let size = [image_rgba.width() as _, image_rgba.height() as _];
                    let pixels = image_rgba.into_flat_samples();
                    let egui_image =
                        egui::ColorImage::from_rgba_unmultiplied(size, pixels.as_slice());
                    self.image_texture = Some(ctx.load_texture(
                        image_path,
                        egui_image,
                        egui::TextureOptions::LINEAR,
                    ));
                }
                _ => std::process::exit(1),
            },
            _ => std::process::exit(1),
        }
    }
    fn new(cc: &egui::Context, image_path: &str) -> Self {
        let mut app = Self::default();
        app.image_path = Some(image_path.to_string());
        app.load_image(cc);
        app
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
    eframe::run_native(
        "My_photo_editor",
        native_options,
        Box::new(|cc| {
            Ok({
                let app = MyApp::new(&cc.egui_ctx, &args[1]);
                Box::new(app)
            })
        }),
    )
    .expect("Unable to open a egui window !");
}
