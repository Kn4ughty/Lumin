use eframe::egui;
use egui::{Key};





fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([320.0, 240.0])
            .with_title("test")
            .with_decorations(false)
            .with_window_level(egui::viewport::WindowLevel::AlwaysOnTop),
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|cc| {
            // This gives us image support
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Ok(Box::<MyApp>::default())
        }),
    )
}

struct MyApp {
    search_text: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            search_text: "".to_owned(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut self.search_text).request_focus();
                
            });

            if ctx.input(|i| i.key_pressed(Key::A)) {
                println!("key press");
            }

            // self.age += 1;
        });
    }
}
