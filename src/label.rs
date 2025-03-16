use egui::{CentralPanel, Color32};
mod widgets;

#[derive(Default)]
struct MyApp {}

impl MyApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            widgets::ColorText::new("Item Value")
                .color(Color32::RED)
                .show(ui);
            widgets::ColorText::new("Item Value")
                .color(Color32::WHITE)
                .show(ui);
            widgets::ColorText::new("Item Value")
                .color(Color32::BLACK)
                .show(ui);
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    eframe::run_native(
        "MyApp",
        eframe::NativeOptions::default(),
        Box::new(|cc| Ok(Box::new(MyApp::new(cc)))),
    )
}
