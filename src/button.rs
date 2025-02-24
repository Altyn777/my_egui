use egui::{
    Button, CentralPanel, Color32, Context, Label, Pos2, RichText, Ui, Vec2, ViewportCommand, Widget
};

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

fn button_demo(ui: &mut Ui, counter: &mut i32,_frame: &mut eframe::Frame) {
    if ui.add(Button::new("Click me")).clicked() {
        println!("Click");
    }
    if ui.small_button("0").clicked() {
        println!("0");
    }
    if Button::new("print 2").ui(ui).clicked() {
        println!("4");
    }
    if ui.add_enabled(false, Button::new("Can't")).clicked() {
        unreachable!();
    }
    if ui.button(RichText::new("Quit").color(Color32::BLUE)).clicked() {
        println!("1");
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
            button_demo(ui,  &mut self.counter, frame);
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let viewport = egui::ViewportBuilder {
        position: Some(Pos2 { x: 0., y:0. }),
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
        Box::new(|cc| Ok(Box::new(ButtonDemo::new(cc))))
    )
}
