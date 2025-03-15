use crate::widgets::{Button, ColorText, Input};
use egui::{Color32, Rect, Response, Ui};

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
            let input_response = Input::new(text_input)
                .enabled(enabled)
                .placeholder(placeholder.unwrap_or_default())
                .show(ui);

            let input_rect = input_response.rect;

            ui.advance_cursor_after_rect(Rect::from_min_size(
                input_rect.min,
                egui::vec2(58.0, input_rect.height()),
            ));

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
