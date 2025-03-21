use eframe::egui;
use egui::Slider;
use my_egui::Gauge;

#[derive(Default)]
struct GaugeExample {
    value: f64,
}

impl GaugeExample {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for GaugeExample {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Gauge Example");
            ui.spacing_mut().slider_width = 300.0;
            ui.add(Slider::new(&mut self.value, 0.0..=100.0));
            ui.add(
                Gauge::new(self.value, 0.0..=100.0, 200.0).angle_range(0..=180), //.show_value(false),
            );
            ui.add(Gauge::new(self.value + 100.0, 100.0..=200.0, 300.0).units("°C"));
        });
    }
}

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Gauge Example",
        native_options,
        Box::new(|cc| Ok(Box::new(GaugeExample::new(cc)))),
    )
    .unwrap();
}
