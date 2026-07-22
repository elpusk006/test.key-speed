mod app;
mod platform;

use app::KeySpeedApp;
use eframe::egui;

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Key-Speed Notepad Logger (Rust Windows & Linux)")
            .with_inner_size([1100.0, 700.0])
            .with_min_inner_size([800.0, 500.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Key-Speed Notepad Logger",
        native_options,
        Box::new(|cc| Ok(Box::new(KeySpeedApp::new(cc)))),
    )
}
