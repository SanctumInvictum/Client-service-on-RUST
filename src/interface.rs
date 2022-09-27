use egui::{SliderOrientation, Ui, Vec2};
use egui::style::Spacing;

pub mod gui;
use gui::TemplateApp;

fn main() {

    tracing_subscriber::fmt::init();

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "GUI",
        native_options,
        Box::new(|cc| Box::new(TemplateApp::new(cc)))
    );
}