use egui::CentralPanel;
use my_egui::{ControlButtonValue, MyProgressBar};

#[derive(Default)]
struct MyApp {
    text1: String,
    text2: String,
}

impl MyApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            if ControlButtonValue::new(&mut self.text1)
                .placeholder("Kitty")
                .label("cat")
                .button_text("Годувати")
                .show(ui)
                .clicked()
            {
                println!("Feed {}", self.text1);
            }

            ControlButtonValue::new(&mut self.text2)
                .enabled(false)
                .show(ui);

            MyProgressBar::new("hi").value(0.6).show(ui)
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
