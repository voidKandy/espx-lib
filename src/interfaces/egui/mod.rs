pub mod integration;
pub mod window;
use eframe::egui;

#[derive(Default)]
struct InitialApp {}

impl eframe::App for InitialApp {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::TRANSPARENT.to_array() // Make sure we don't paint anything behind the rounded corners
    }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        window::custom_window_frame(ctx, frame, "Consoxide", |ui| {
            ui.label("This is just the contents of the window.");
            ui.horizontal(|ui| {
                integration::AgentApp::default().message_window(ctx);
                //     ui.label("egui theme:");
                //     egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });
    }
}

pub fn init() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        decorated: false,
        transparent: true,
        min_window_size: Some(egui::vec2(1280.0, 640.0)),
        initial_window_size: Some(egui::vec2(1280.0, 640.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Consoxide",
        options,
        Box::new(|_cc| Box::<InitialApp>::default()),
    )
}