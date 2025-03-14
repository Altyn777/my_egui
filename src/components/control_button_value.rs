use crate::widgets::{Button, ColorText, Input};
use egui::{Response, Ui};

pub struct ControlButtonValue<'a> {
    text_input: &'a mut String,
    enabled: bool,
    placeholder: Option<String>,
}

impl<'a> ControlButtonValue<'a> {
    pub fn new(text_input: &'a mut String) -> Self {
        Self {
            text_input,
            enabled: true,
            placeholder: None,
        }
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    pub fn show(self, ui: &mut Ui) -> Response {
        let ControlButtonValue {
            text_input,
            enabled,
            placeholder,
        } = self;

        ui.horizontal(|ui| {
            let mut input = Input::new(text_input).enabled(enabled);

            if let Some(placeholder_text) = placeholder {
                input = input.placeholder(placeholder_text);
            }

            input.show(ui);

            ColorText::new("cat")
                .color(egui::Color32::from_rgb(150, 150, 150))
                .show(ui);

            let button_response = Button::new("Годувати")
                .enabled(enabled && !text_input.is_empty())
                .show(ui);

            button_response
        })
        .inner
    }
}
