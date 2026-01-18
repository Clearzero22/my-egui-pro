mod app;
mod category;
mod hn_api;
mod storage;
mod story;
mod ui;

use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1000.0, 700.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Hacker News Reader",
        options,
        Box::new(|_cc| Ok(Box::new(app::HackerNewsApp::new()))),
    )
}
