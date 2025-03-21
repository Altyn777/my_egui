use egui::{Color32, Response, RichText, Ui};

pub struct MyProgressBar<'a> {
    text: &'a str,
    value: f32,
    font_size: f32,
    min: f32,
    max: f32,
}

impl<'a> MyProgressBar<'a> {
    pub fn new(text: &'a str) -> Self {
        Self {
            text,
            value: 0.0,
            font_size: 14.0,
            min: 0.0,
            max: 100.0,
        }
    }

    pub fn value(mut self, value: f32) -> Self {
        self.value = value;
        self
    }

    pub fn font_size(mut self, font_size: f32) -> Self {
        self.font_size = font_size;
        self
    }

    pub fn show(self, ui: &mut Ui) -> Response {
        ui.horizontal(|ui| {
            ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                let min_text = RichText::new(self.min.to_string())
                    .color(Color32::from_rgb(169, 169, 169))
                    .size(10.0);
                ui.label(min_text);

                ui.vertical(|ui| {
                    ui.scope(|ui| {
                        ui.set_width(180.0);
                        let progress_bar = egui::ProgressBar::new(self.value)
                            .fill(Color32::GREEN)
                            .desired_height(5.0);
                        ui.add(progress_bar);

                        ui.with_layout(
                            egui::Layout::top_down_justified(egui::Align::Center),
                            |ui| {
                                let rich_text =
                                    RichText::new(self.text).color(Color32::WHITE).size(16.0);
                                ui.label(rich_text);
                            },
                        );
                    });
                });

                let max_text = RichText::new(self.max.to_string())
                    .color(Color32::from_rgb(169, 169, 169))
                    .size(10.0);
                ui.label(max_text);
            });
        })
        .response
    }
}
