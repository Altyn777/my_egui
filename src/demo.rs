use egui::CentralPanel;
mod components;
mod widgets;

#[derive(Default)]
struct MyApp {
    text1: String,
    text2: String,
    text3: String,
}

impl MyApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            widgets::Input::new(&mut self.text1)
                .placeholder("Введіть ваше ім'я...")
                .show(ui);

            if components::ControlButtonValue::new(&mut self.text2)
                .placeholder(&String::from("Kitty"))
                .show(ui)
                .clicked()
            {
                println!("Feed {}", self.text2);
            }

            components::ControlButtonValue::new(&mut self.text3)
                .enabled(false)
                .show(ui)
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
