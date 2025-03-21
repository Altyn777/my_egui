use egui::{CentralPanel, Context, Pos2, Ui, Vec2};
use my_egui::Button;

#[derive(Default)]
struct ButtonDemo {
    counter: i32,
}

impl ButtonDemo {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }
}

fn button_demo(ui: &mut Ui, _counter: &mut i32, _frame: &mut eframe::Frame) {
    Button::new("Кнопка").show(ui);

    Button::new("Вимкнена кнопка").enabled(false).show(ui);

    let button = Button::new("Натисни мене").enabled(true);

    if button.show(ui).clicked() {
        println!("Кнопка натиснута!");
    }
}

impl eframe::App for ButtonDemo {
    fn update(&mut self, ctx: &Context, frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.add_space(10.0);
            button_demo(ui, &mut self.counter, frame);
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let viewport = egui::ViewportBuilder {
        position: Some(Pos2 { x: 0., y: 0. }),
        inner_size: Some(Vec2 { x: 200., y: 200. }),
        ..Default::default()
    };
    let options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };
    eframe::run_native(
        "Button Demo",
        options,
        Box::new(|cc| Ok(Box::new(ButtonDemo::new(cc)))),
    )
}
