use egui::{Color32, Response, TextEdit, Ui, Vec2};

pub struct Input<'a> {
    text: &'a mut String,
    placeholder: Option<String>,
    enabled: bool,
}

impl<'a> Input<'a> {
    pub fn new(text: &'a mut String) -> Self {
        Self {
            text,
            placeholder: None,
            enabled: true,
        }
    }

    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    pub fn show(self, ui: &mut Ui) -> Response {
        let Input {
            text,
            placeholder,
            enabled,
        } = self;

        let is_empty = text.is_empty();

        let original_button_padding = ui.spacing().button_padding;
        ui.spacing_mut().button_padding = Vec2::new(8.0, 8.0);

        let mut text_edit = TextEdit::singleline(text).frame(true).interactive(enabled);

        if let Some(placeholder_text) = placeholder {
            if is_empty {
                text_edit = text_edit.hint_text(placeholder_text);
            }
        }

        if !enabled {
            text_edit = text_edit.text_color(Color32::from_rgb(120, 120, 120));
        }

        let response = ui.add(text_edit);

        ui.spacing_mut().button_padding = original_button_padding;

        response
    }
}
