use eframe::egui;
use std::time::{Instant, Duration};

pub struct MyApp {
    counter: i32,
    last_change: Instant,
    last_frame: Instant,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            counter: 0,
            last_change: Instant::now(),
            last_frame: Instant::now(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let now = Instant::now();
        let time_since_change = now.duration_since(self.last_change);
        
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(100.0);
                ui.heading(egui::RichText::new("Hello World!").size(48.0).color(egui::Color32::WHITE));
                ui.add_space(30.0);
                
                if time_since_change < Duration::from_secs(1) {
                    ui.colored_label(egui::Color32::GREEN, "▶ Updated!");
                }
                
                ui.add_space(10.0);
                ui.label(egui::RichText::new(format!("Counter: {}", self.counter)).size(24.0));
                ui.add_space(30.0);
                
                let button = ui.button(egui::RichText::new("Click Me!").size(20.0));
                
                if button.clicked() {
                    self.counter += 1;
                    self.last_change = now;
                    // HANYA repaint saat tombol diklik
                    ctx.request_repaint();
                }
                
                let elapsed = now.duration_since(self.last_frame);
                self.last_frame = now;
                ui.add_space(50.0);
                ui.colored_label(egui::Color32::from_gray(100), 
                    format!("Frame time: {:.2}ms (idle = no repaint)", elapsed.as_secs_f64() * 1000.0));
            });
        });
        
        // Tidak panggil request_repaint_after karena itu tidak ada di API
        // Biarkan egui mengatur default repaint interval
    }
}
