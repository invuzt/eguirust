use eframe::egui;

pub struct MyApp {
    counter: i32,
    last_interaction: bool,
    frame_count: u32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            counter: 0,
            last_interaction: false,
            frame_count: 0,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.frame_count += 1;
        
        // Debug: print hanya 10 frame pertama untuk bukti
        if self.frame_count <= 10 {
            println!("Frame {} - UI rendered", self.frame_count);
        }
        
        let mut needs_repaint = false;
        
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(100.0);
                ui.heading(egui::RichText::new("Hello World!").size(48.0).color(egui::Color32::WHITE));
                ui.add_space(20.0);
                
                // Tampilkan status
                if self.last_interaction {
                    ui.colored_label(egui::Color32::GREEN, "▶ UI aktif (ada interaksi)");
                } else {
                    ui.colored_label(egui::Color32::GRAY, "⏸ UI idle (tidak dirender)");
                }
                
                ui.add_space(10.0);
                ui.label(egui::RichText::new(format!("Counter: {}", self.counter)).size(24.0));
                ui.add_space(30.0);
                
                // Tombol - hanya ini yang trigger repaint
                let button = ui.button(egui::RichText::new("Click Me!").size(20.0));
                
                if button.clicked() {
                    self.counter += 1;
                    needs_repaint = true;
                    self.last_interaction = true;
                }
                
                // Deteksi interaksi lain (drag, hover, dll)
                if button.hovered() || button.dragged() {
                    needs_repaint = true;
                    self.last_interaction = true;
                }
            });
        });
        
        // Deteksi input dari user (touch, keyboard)
        let has_input = ctx.input(|i| {
            i.pointer.any_down() ||           // Jari menyentuh layar
            i.pointer.button_clicked() ||     // Klik/tap
            i.pointer.any_pressed() ||        // Tekanan baru
            i.keys_down.len() > 0 ||          // Keyboard tertekan
            i.zoom_delta() != 1.0 ||          // Zoom gesture
            i.scroll_delta != egui::Vec2::ZERO // Scroll
        });
        
        if has_input {
            needs_repaint = true;
            self.last_interaction = true;
        }
        
        // Reset status idle setelah beberapa detik
        if self.last_interaction && !has_input {
            // Simulasi: setelah 2 detik tanpa interaksi, masuk idle
            // Di egui kita simpan timestamp untuk yang lebih akurat
            needs_repaint = true;
            self.last_interaction = false;
        }
        
        // HANYA repaint kalau ada perubahan atau interaksi
        if needs_repaint {
            ctx.request_repaint();
        }
        
        // Atau pakai timer repaint (render tiap 0.5 detik saat idle)
        // ctx.request_repaint_after(std::time::Duration::from_millis(500));
    }
}
