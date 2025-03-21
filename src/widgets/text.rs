use egui::{Color32, Frame, Margin, Response, RichText, Ui};

pub struct ColorText {
    text: &'static str,
    color: Color32,
    size: Option<f32>,
}

impl ColorText {
    pub fn new(text: &'static str) -> Self {
        Self {
            text,
            color: Color32::WHITE,
            size: Some(14.0),
        }
    }

    pub fn color(mut self, color: Color32) -> Self {
        self.color = color;
        self
    }

    pub fn size(mut self, size: f32) -> Self {
        self.size = Some(size);
        self
    }

    pub fn show(self, ui: &mut Ui) -> Response {
        let ColorText { text, color, size } = self;

        let color = if ui.visuals().dark_mode {
            Color32::WHITE
        } else {
            Color32::BLACK
        };

        let mut rich_text = RichText::new(text).color(color);

        if let Some(text_size) = size {
            rich_text = rich_text.size(text_size);
        }

        let margin = Margin::same(8);

        Frame::new()
            .inner_margin(margin)
            .show(ui, |ui| ui.label(rich_text))
            .response
    }
}
