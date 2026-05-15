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
        let screen_rect = ctx.input(|i| i.screen_rect());
        let screen_width = screen_rect.width();
        let screen_height = screen_rect.height();
        
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(screen_height * 0.2);
                
                ui.heading(egui::RichText::new("Hello World!").size(48.0).color(egui::Color32::WHITE));
                ui.add_space(30.0);
                
                let button_width = (screen_width / 3.0).min(200.0);
                let button = egui::Button::new(egui::RichText::new(format!("Click Me! ({})", self.counter)).size(24.0))
                    .fill(egui::Color32::from_rgb(34, 197, 94))
                    .rounding(egui::Rounding::same(16.0));
                
                if ui.add_sized(egui::vec2(button_width, 60.0), button).clicked() {
                    self.counter += 1;
                }
                
                ui.add_space(20.0);
                ui.label(egui::RichText::new("Vuzt - Android Rust App").size(14.0).color(egui::Color32::from_gray(150)));
                ui.label(egui::RichText::new("Egui version 0.27 - Stable").size(12.0).color(egui::Color32::from_gray(100)));
            });
        });
    }
}
