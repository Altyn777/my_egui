use egui::{Color32, Response, RichText, Sense, Stroke, Ui, Vec2};

pub struct Button {
    text: String,
    enabled: bool,
}

impl Button {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            enabled: true,
        }
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    pub fn show(self, ui: &mut Ui) -> Response {
        let Button { text, enabled } = self;

        let original_button_padding = ui.spacing().button_padding;
        ui.spacing_mut().button_padding = Vec2::new(8.0, 8.0);
        let prev_visuals = ui.style().visuals.clone();
        let mut visuals = prev_visuals.clone();
        let border_stroke = Stroke::new(1.0, Color32::TRANSPARENT);
        visuals.widgets.inactive.bg_stroke = border_stroke;
        visuals.widgets.hovered.bg_stroke = border_stroke;
        visuals.widgets.active.bg_stroke = border_stroke;
        ui.style_mut().visuals = visuals;

        let rich_text = RichText::new(text).color(Color32::WHITE).size(14.0);

        let background_color = if enabled {
            Color32::from_rgb(0, 133, 255)
        } else {
            Color32::from_rgb(169, 169, 169)
        };

        let mut button = egui::Button::new(rich_text).fill(background_color);

        if !enabled {
            button = button.sense(Sense::hover());
        }

        let response = ui.add(button);

        ui.spacing_mut().button_padding = original_button_padding;
        ui.style_mut().visuals = prev_visuals;

        response
    }
}
