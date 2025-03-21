use egui::{Color32, Rect, Response, Ui};

use crate::widgets::{Button, ColorText, Input};

const INPUT_WIDTH: f32 = 58.0;

pub struct ControlButtonValue<'a> {
    text_input: &'a mut String,
    label: Option<&'static str>,
    button_text: &'a str,
    enabled: bool,
    placeholder: Option<&'static str>,
}

impl<'a> ControlButtonValue<'a> {
    pub fn new(text_input: &'a mut String) -> Self {
        Self {
            text_input,
            label: None,
            enabled: true,
            button_text: "Apply",
            placeholder: None,
        }
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    pub fn placeholder(mut self, placeholder: &'static str) -> Self {
        self.placeholder = Some(placeholder);
        self
    }

    pub fn label(mut self, label: &'static str) -> Self {
        self.label = Some(label);
        self
    }

    pub fn button_text(mut self, button_text: &'a str) -> Self {
        self.button_text = button_text;
        self
    }

    pub fn show(self, ui: &mut Ui) -> Response {
        let ControlButtonValue {
            text_input,
            button_text,
            label,
            enabled,
            placeholder,
        } = self;

        ui.horizontal(|ui| {
            let input_response = Input::new(text_input)
                .enabled(enabled)
                .placeholder(placeholder)
                .show(ui);

            let input_rect = input_response.rect;
            ui.advance_cursor_after_rect(Rect::from_min_size(
                input_rect.min,
                egui::vec2(INPUT_WIDTH, input_rect.height()),
            ));

            if let Some(label) = label {
                ColorText::new(label)
                    .color(Color32::from_rgb(150, 150, 150))
                    .show(ui);
            }

            Button::new(button_text)
                .enabled(enabled && !text_input.is_empty())
                .show(ui)
        })
        .inner
    }
}
