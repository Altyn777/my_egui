use egui::{Color32, Margin, Response, Stroke, TextEdit, Ui};

pub struct Input<'a> {
    text: &'a mut String,
    placeholder: Option<&'static str>,
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

    pub fn placeholder(mut self, placeholder: Option<&'static str>) -> Self {
        self.placeholder = placeholder;
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

        let original_widgets = ui.style().visuals.widgets.clone();
        let original_selection = ui.style().visuals.selection;
        let widgets = &mut ui.style_mut().visuals.widgets;

        let border_color = Color32::from_rgb(0, 133, 255);
        let border_stroke = Stroke::new(1.0, border_color);

        widgets.noninteractive.bg_stroke = Stroke::new(1.0, Color32::from_rgb(255, 255, 255));
        widgets.inactive.bg_stroke = border_stroke;
        widgets.active.bg_stroke = border_stroke;
        widgets.hovered.bg_stroke = border_stroke;
        widgets.open.bg_stroke = border_stroke;

        ui.style_mut().visuals.selection.stroke = border_stroke;

        let mut text_edit = TextEdit::singleline(text)
            .clip_text(true)
            .desired_width(58.0)
            .margin(Margin::same(8))
            .text_color(Color32::WHITE)
            .background_color(Color32::from_rgb(5, 9, 25))
            .interactive(enabled);

        if let Some(placeholder_text) = placeholder {
            if is_empty {
                text_edit = text_edit.hint_text(placeholder_text);
            }
        }

        let response = ui.add(text_edit);

        ui.style_mut().visuals.widgets = original_widgets;
        ui.style_mut().visuals.selection = original_selection;

        response
    }
}
