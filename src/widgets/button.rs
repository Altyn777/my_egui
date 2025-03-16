use egui::{Color32, Response, RichText, Sense, Stroke, Ui, Vec2};

pub struct Button<'a> {
    text: &'a str,
    enabled: bool,
    font_size: f32,
}

impl<'a> Button<'a> {
    pub fn new(text: &'a str) -> Self {
        Self {
            text,
            enabled: true,
            font_size: 14.0,
        }
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    pub fn font_size(mut self, font_size: f32) -> Self {
        self.font_size = font_size;
        self
    }

    pub fn show(self, ui: &mut Ui) -> Response {
        let original_button_padding = ui.spacing().button_padding;
        ui.spacing_mut().button_padding = Vec2::new(8.0, 8.0);

        let original_widgets = ui.style().visuals.widgets.clone();
        let widgets = &mut ui.style_mut().visuals.widgets;

        let border_stroke = Stroke::new(1.0, Color32::TRANSPARENT);
        widgets.inactive.bg_stroke = border_stroke;
        widgets.hovered.bg_stroke = border_stroke;
        widgets.active.bg_stroke = border_stroke;

        let rich_text = RichText::new(self.text).color(Color32::WHITE).size(14.0);

        let background_color = if self.enabled {
            Color32::from_rgb(0, 133, 255)
        } else {
            Color32::from_rgb(169, 169, 169)
        };

        let mut button = egui::Button::new(rich_text).fill(background_color);

        if !self.enabled {
            button = button.sense(Sense::hover());
        }

        let response = ui.add(button);

        ui.spacing_mut().button_padding = original_button_padding;
        ui.style_mut().visuals.widgets = original_widgets;

        response
    }
}
