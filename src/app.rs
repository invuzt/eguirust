use eframe::egui;

pub struct MyApp {
    counter: i32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self { counter: 0 }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(100.0);
                ui.heading(egui::RichText::new("Hello World!").size(48.0).color(egui::Color32::WHITE));
                ui.add_space(30.0);
                ui.label(egui::RichText::new(format!("Counter: {}", self.counter)).size(24.0));
                ui.add_space(30.0);
                if ui.button(egui::RichText::new("Click Me!").size(20.0)).clicked() {
                    self.counter += 1;
                }
            });
        });
    }
}
