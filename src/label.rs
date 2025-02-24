use egui::{ CentralPanel, Label };

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
            ui.label("ui.label");
            ui.heading("ui.heading");
            ui.add(Label::new("ui.add"));
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    eframe::run_native(
        "MyApp",
        eframe::NativeOptions::default(),
        Box::new(|cc| Ok(Box::new(MyApp::new(cc))))
    )
}
