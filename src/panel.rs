use egui::{
    menu, CentralPanel, Context, Label, SidePanel, TopBottomPanel, ViewportBuilder, Visuals, Window
};

#[derive(Default)]
struct PanelDemo {
}

impl PanelDemo {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(Visuals::dark());
        Default::default()
    }
}

impl eframe::App for PanelDemo {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        TopBottomPanel::top("top_panel_0").show(ctx, |ui| {
            ui.label("Top 0");
            menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Save").clicked() {
                        println!("save");
                    }
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                
                ui.menu_button("Edit", |ui| {
                    if ui.button("Cut").clicked() {
                        println!("cut");
                    }
                    if ui.button("Paste").clicked() {
                        println!("paste");
                    }
                });
            })
        });
        SidePanel::left("side_left_panel").show(ctx, |ui| {
            ui.label("Side left");
        });
        TopBottomPanel::top("top_panel_1").show(ctx, |ui| {
            ui.label("Top 1");
        });
        SidePanel::right("side_right_panel_0").show(ctx, |ui| {
            ui.label("Side right 0");
        });
        SidePanel::right("side_right_panel_1").show(ctx, |ui| {
            ui.label("Side right 1");
        });

        CentralPanel::default().show(ctx, |ui| {
            ui.add_space(10.0);
            Window::new("Window 0").show(ctx, |ui| {
                ui.add(Label::new("Window 0"));
            });
            Window::new("Window 1").show(ctx, |ui| {
                ui.add(Label::new("Window 1"));
            });
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let viewport = ViewportBuilder::default().with_inner_size([600., 200.]);
    let options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };
    eframe::run_native(
        "Panel Demo",
        options,
        Box::new(|cc| Ok(Box::new(PanelDemo::new(cc))))
    )
}
