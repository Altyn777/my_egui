use egui::{Color32, Response, RichText, Ui};

pub struct ColorText {
    text: String,
    color: Color32,
    size: Option<f32>,
}

impl ColorText {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            color: Color32::BLACK,
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

        let mut rich_text = RichText::new(text).color(color);

        if let Some(text_size) = size {
            rich_text = rich_text.size(text_size);
        }

        ui.label(rich_text)
    }
}
