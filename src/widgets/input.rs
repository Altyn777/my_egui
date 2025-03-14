use egui::{Color32, Response, TextEdit, Ui, Vec2};

pub struct Input {
    text: String,
    placeholder: Option<String>,
    enabled: bool,
}

impl Input {
    pub fn new(text: &mut String) -> Self {
        Self {
            text: text.clone(),
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

        let border_color = if enabled {
            Color32::from_rgb(100, 100, 100)
        } else {
            Color32::from_rgb(60, 60, 60)
        };

        let mut text_edit = TextEdit::singleline(&text).frame(true).interactive(enabled);

        if let Some(placeholder_text) = placeholder {
            if text.is_empty() {
                text_edit = text_edit.hint_text(placeholder_text);
            }
        }

        let frame_visuals = ui.visuals().widgets.inactive;
        let mut custom_frame = frame_visuals.clone();
        custom_frame.bg_stroke.color = border_color;

        let old_visuals = ui.visuals().clone();
        let mut visuals = ui.visuals().clone();
        visuals.widgets.inactive = custom_frame;
        ui.set_visuals(visuals);

        let original_button_padding = ui.spacing().button_padding;

        ui.spacing_mut().button_padding = Vec2::new(8.0, 8.0);

        let response = ui.add(text_edit);

        ui.set_visuals(old_visuals);
        ui.spacing_mut().button_padding = original_button_padding;

        response
    }
}
