use eframe::egui;
use egui_keyboard::{Keyboard, KeyboardLayout};

pub struct MyApp {
    text: String,
    show_keyboard: bool,
    keyboard: Keyboard,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            text: String::new(),
            show_keyboard: false,
            keyboard: Keyboard::new(KeyboardLayout::AZERTY),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Update keyboard setiap frame
        self.keyboard.update(ctx);
        
        let screen_rect = ctx.input(|i| i.screen_rect());
        let screen_height = screen_rect.height();
        
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(screen_height * 0.15);
                
                ui.heading("Vuzt Keyboard Demo");
                ui.add_space(20.0);
                
                ui.label("Input Teks dengan Virtual Keyboard:");
                ui.add_space(10.0);
                
                // Text field
                let text_edit = egui::TextEdit::singleline(&mut self.text)
                    .hint_text("Ketik di sini...")
                    .desired_width(300.0)
                    .font(egui::FontId::proportional(18.0));
                
                if ui.add(text_edit).clicked() {
                    self.show_keyboard = true;
                }
                
                ui.add_space(20.0);
                
                if !self.text.is_empty() {
                    ui.label(format("Output: {}", self.text));
                }
                
                ui.add_space(30.0);
                
                // Tombol untuk membuka keyboard
                if ui.button("Buka Keyboard").clicked() {
                    self.show_keyboard = true;
                }
                
                ui.add_space(10.0);
                
                if ui.button("Clear").clicked() {
                    self.text.clear();
                }
            });
        });
        
        // Tampilkan keyboard jika diperlukan
        if self.show_keyboard {
            egui::Window::new("Keyboard")
                .collapsible(false)
                .resizable(false)
                .title_bar(true)
                .anchor(egui::Align2::CENTER_BOTTOM, [0.0, -10.0])
                .show(ctx, |ui| {
                    self.keyboard.show_ui(ui);
                    
                    ui.add_space(10.0);
                    ui.horizontal(|ui| {
                        if ui.button("Tutup").clicked() {
                            self.show_keyboard = false;
                        }
                        if ui.button("Clear Input").clicked() {
                            self.text.clear();
                        }
                    });
                });
        }
    }
}
