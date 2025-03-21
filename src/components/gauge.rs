use egui::epaint::PathShape;
use egui::{Align2, Color32, FontFamily, FontId, Pos2, Rect, Response, Sense, Shape, Stroke, Ui};
use std::f32::consts::PI;
use std::ops::RangeInclusive;

pub struct Gauge {
    value: f64,
    value_range: RangeInclusive<f64>,
    size: f32,
    angle_range: RangeInclusive<i16>,
    stroke_width: f32,
    units: String,
    display_value: bool,
    bg_color: Color32,
    fg_color: Color32,
    text_color: Option<Color32>,
    tick_count: usize,
    tick_size: f32,
    pointer_radius: f32,
}

impl Default for Gauge {
    fn default() -> Self {
        Self {
            value: 0.0,
            value_range: 0.0..=100.0,
            size: 200.0,
            angle_range: -45..=225,
            stroke_width: 1.5,
            units: String::new(),
            display_value: true,
            bg_color: Color32::from_rgb(47, 47, 47),
            fg_color: Color32::from_rgb(0, 190, 42),
            text_color: None,
            tick_count: 7,
            tick_size: 3.0,
            pointer_radius: 3.0,
        }
    }
}

impl Gauge {
    pub fn new<V, R>(value: V, range: R, size: f32) -> Self
    where
        V: Into<f64>,
        R: Into<RangeInclusive<f64>>,
    {
        let value_range = range.into();
        let value = value.into().clamp(*value_range.start(), *value_range.end());

        Self {
            value,
            value_range,
            size,
            ..Default::default()
        }
    }

    pub fn angle_range(mut self, angle_range: RangeInclusive<i16>) -> Self {
        let start = angle_range.start().clamp(&-360, &360);
        let end = angle_range.end().clamp(&-360, &360);
        self.angle_range = *start..=*end;
        self
    }

    pub fn display_value(mut self, display_value: bool) -> Self {
        self.display_value = display_value;
        self
    }

    pub fn units(mut self, units: impl Into<String>) -> Self {
        self.units = units.into();
        self
    }

    pub fn bg_color(mut self, color: Color32) -> Self {
        self.bg_color = color;
        self
    }

    pub fn fg_color(mut self, color: Color32) -> Self {
        self.fg_color = color;
        self
    }

    pub fn text_color(mut self, color: Color32) -> Self {
        self.text_color = Some(color);
        self
    }

    pub fn tick_count(mut self, count: usize) -> Self {
        self.tick_count = count.max(2);
        self
    }

    pub fn pointer_radius(mut self, size: f32) -> Self {
        self.pointer_radius = size.max(1.0);
        self
    }

    pub fn set_value(&mut self, value: f64) {
        self.value = value.clamp(*self.value_range.start(), *self.value_range.end());
    }

    fn gauge_width(&self) -> f32 {
        self.size - self.text_clearance() * 2.0
    }

    fn text_clearance(&self) -> f32 {
        self.size / 10.0
    }

    fn position_from_angle(&self, rect: Rect, angle: i16, radius: f32) -> Pos2 {
        let center = rect.center();
        let angle_rad = angle as f32 * PI / 180.0;
        center + egui::vec2(angle_rad.cos() * radius, -angle_rad.sin() * radius)
    }

    fn radius(&self) -> f32 {
        self.gauge_width() / 2.0
    }

    fn value_to_angle(&self, v: f64) -> i16 {
        let max_angle = *self.angle_range.end();
        let min_angle = *self.angle_range.start();
        let angle_range = (max_angle - min_angle) as f64;
        let min_value = self.value_range.start();
        let max_value = self.value_range.end();
        let normalized = (v - min_value) / (max_value - min_value);
        (max_angle as f64 - (normalized * angle_range)) as i16
    }

    fn paint(&mut self, ui: &mut Ui, outer_rect: Rect) {
        let padding = self.text_clearance();
        let rect = outer_rect.shrink(padding);

        let min_angle = *self.angle_range.start();
        let max_angle = *self.angle_range.end();
        let current_angle = self.value_to_angle(self.value);

        self.paint_arc(ui, rect, min_angle, max_angle, self.bg_color);
        self.paint_arc(ui, rect, current_angle, max_angle, self.fg_color);

        self.paint_point(ui, rect, current_angle);
        self.paint_point(ui, rect, max_angle);

        self.paint_ticks(ui, rect);

        if self.display_value {
            self.paint_current_value(ui, rect);
        }
    }

    fn paint_arc(&self, ui: &mut Ui, rect: Rect, start_angle: i16, end_angle: i16, color: Color32) {
        if start_angle >= end_angle {
            return;
        }

        let step = ((end_angle - start_angle) / 30).max(1);
        let mut points = Vec::with_capacity((end_angle - start_angle) as usize / step as usize + 1);

        let mut angle = start_angle;
        while angle <= end_angle {
            points.push(self.position_from_angle(rect, angle, self.radius()));
            angle += step;
        }

        if angle - step < end_angle {
            points.push(self.position_from_angle(rect, end_angle, self.radius()));
        }

        if !points.is_empty() {
            ui.painter().add(Shape::Path(PathShape {
                points,
                closed: false,
                fill: Color32::TRANSPARENT,
                stroke: Stroke::new(self.stroke_width, color).into(),
            }));
        }
    }

    fn paint_ticks(&self, ui: &mut Ui, rect: Rect) {
        let text_color = self.text_color.unwrap_or_else(|| {
            if ui.visuals().dark_mode {
                Color32::from_rgb(169, 169, 169)
            } else {
                Color32::GRAY
            }
        });

        let value_range = *self.value_range.end() - *self.value_range.start();
        let step = value_range / (self.tick_count - 1) as f64;
        let font_size = self.gauge_width() / 15.0;

        for i in 0..self.tick_count {
            let tick_value = *self.value_range.start() + step * i as f64;
            let angle = self.value_to_angle(tick_value);

            // let tick_inner = self.position_from_angle(rect, angle, self.radius() - self.tick_size);
            // let tick_outer = self.position_from_angle(rect, angle, self.radius() + self.tick_size);
            // ui.painter()
            //     .line_segment([tick_inner, tick_outer], Stroke::new(1.0, text_color));

            let text_pos =
                self.position_from_angle(rect, angle, self.radius() + self.gauge_width() * 0.1);
            ui.painter().text(
                text_pos,
                Align2::CENTER_CENTER,
                format!("{}", tick_value as i32),
                FontId::new(font_size, FontFamily::default()),
                text_color,
            );
        }
    }

    fn paint_current_value(&self, ui: &mut Ui, rect: Rect) {
        let text_color = self.text_color.unwrap_or_else(|| {
            if ui.visuals().dark_mode {
                Color32::WHITE
            } else {
                Color32::GRAY
            }
        });

        let value_text = if self.units.is_empty() {
            format!("{:.1}", self.value)
        } else {
            format!("{:.1} {}", self.value, self.units)
        };

        ui.painter().text(
            rect.center(),
            Align2::CENTER_CENTER,
            value_text,
            FontId::new(self.gauge_width() / 9.0, FontFamily::default()),
            text_color,
        );
    }

    fn paint_point(&self, ui: &mut Ui, rect: Rect, angle: i16) {
        let point = self.position_from_angle(rect, angle, self.radius() - self.stroke_width / 2.0);
        ui.painter().circle(
            point,
            self.pointer_radius,
            self.fg_color,
            Stroke::new(1.0, self.fg_color),
        );
    }
}

impl egui::Widget for Gauge {
    fn ui(mut self, ui: &mut Ui) -> Response {
        let desired_size = egui::vec2(self.size, self.size);
        let (rect, response) = ui.allocate_exact_size(desired_size, Sense::hover());

        response.widget_info(|| egui::WidgetInfo::slider(true, self.value, &self.units));

        if ui.is_rect_visible(rect) {
            self.paint(ui, rect);
        }

        response
    }
}
