mod gui;
mod auth;
mod config;
mod launch;
mod integrity;
mod session;
mod other;

fn main() {
    let app = gui::MicrolaunchApp::default();
    let native_options = eframe::NativeOptions {
        decorated: true,
        transparent: false,
        min_window_size: Some(egui::vec2(320.0, 10.0)),
        ..Default::default()
    };
    eframe::run_native(Box::new(app), native_options);
}