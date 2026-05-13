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
        // Deteksi interaksi sederhana
        let has_interaction = ctx.input(|i| {
            i.pointer.any_down() ||           
            i.pointer.any_pressed() ||
            i.zoom_delta() != 1.0 ||
            i.raw_scroll_delta != egui::Vec2::ZERO
        });
        
        if has_interaction {
            ctx.request_repaint();
        }
        
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(100.0);
                ui.heading(egui::RichText::new("Hello World!").size(48.0).color(egui::Color32::WHITE));
                ui.add_space(30.0);
                ui.label(egui::RichText::new(format!("Counter: {}", self.counter)).size(24.0));
                ui.add_space(30.0);
                
                let button = ui.button(egui::RichText::new("Click Me!").size(20.0));
                if button.clicked() {
                    self.counter += 1;
                    ctx.request_repaint();
                }
            });
        });
    }
}
