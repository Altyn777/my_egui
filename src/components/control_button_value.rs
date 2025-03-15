use crate::widgets::{Button, ColorText, Input};
use egui::{Color32, Response, Ui};

pub struct ControlButtonValue<'a> {
    text_input: &'a mut String,
    label: Option<&'a String>,
    enabled: bool,
    placeholder: Option<String>,
}

impl<'a> ControlButtonValue<'a> {
    pub fn new(text_input: &'a mut String) -> Self {
        Self {
            text_input,
            label: None,
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

    pub fn label(mut self, label: &'a String) -> Self {
        self.label = Some(label);
        self
    }

    pub fn show(self, ui: &mut Ui) -> Response {
        let ControlButtonValue {
            text_input,
            label,
            enabled,
            placeholder,
        } = self;

        ui.horizontal(|ui| {
            let mut input = Input::new(text_input).enabled(enabled);

            if let Some(placeholder_text) = placeholder {
                input = input.placeholder(placeholder_text);
            }

            input.show(ui);

            if let Some(label) = label {
                ColorText::new(label)
                    .color(Color32::from_rgb(150, 150, 150))
                    .show(ui);
            }

            Button::new("Годувати")
                .enabled(enabled && !text_input.is_empty())
                .show(ui)
        })
        .inner
    }
}
