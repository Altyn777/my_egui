use crate::widgets::{Button, ColorText, Input};
use egui::{Response, Ui};

pub struct ControlButtonValue<'a> {
    text: &'a mut String,
    enabled: bool,
}

impl<'a> ControlButtonValue<'a> {
    pub fn new(text: &'a mut String) -> Self {
        Self {
            text,
            enabled: true,
        }
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    pub fn show(self, ui: &mut Ui) -> Response {
        let ControlButtonValue { text, enabled } = self;
        ui.vertical(|ui| {
            let input_response = Input::new(text)
                .placeholder("type cat")
                .enabled(enabled)
                .show(ui);

            ui.add_space(8.0);

            ColorText::new("cat")
                .color(egui::Color32::from_rgb(150, 150, 150))
                .show(ui);

            ui.add_space(16.0);

            let button_response = Button::new("Годувати")
                .enabled(enabled && !text.is_empty())
                .show(ui);

            button_response
        })
        .inner
    }
}
