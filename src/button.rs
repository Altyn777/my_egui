use egui::{Button, CentralPanel, Context, Label, Pos2, Ui, Vec2, ViewportCommand};
mod widgets;

#[derive(Default)]
struct ButtonDemo {
    counter: i32,
}

impl ButtonDemo {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }
    fn ui_counter(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            if ui.add_sized([50., 30.], Button::new("-")).clicked() {
                self.counter -= 1;
            }

            ui.add_sized([50., 30.], Label::new(self.counter.to_string()));

            if ui.add_sized([50., 30.], Button::new("+")).clicked() {
                self.counter += 1;
            }
        });
    }
}

fn button_demo(ui: &mut Ui, counter: &mut i32, _frame: &mut eframe::Frame) {
    widgets::Button::new("Кнопка").show(ui);

    widgets::Button::new("Вимкнена кнопка")
        .enabled(false)
        .show(ui);

    let button = widgets::Button::new("Натисни мене").enabled(true);

    if button.show(ui).clicked() {
        println!("Кнопка натиснута!");
    }
}

impl eframe::App for ButtonDemo {
    fn update(&mut self, ctx: &Context, frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            self.ui_counter(ui);
            ui.add_space(10.0);
            if ui.add_sized([165., 30.], Button::new("Quit")).clicked() {
                ctx.send_viewport_cmd(ViewportCommand::Close);
            }
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
