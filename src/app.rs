use eframe::egui;

pub struct MyApp {
    input_text: String,
    show_keyboard: bool,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            input_text: String::new(),
            show_keyboard: false,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let screen_rect = ctx.input(|i| i.screen_rect());
        
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(80.0);
                ui.heading(egui::RichText::new("Android Style Keyboard").size(28.0).color(egui::Color32::WHITE));
                ui.add_space(30.0);
                
                // Text field seperti Android
                let text_edit = egui::TextEdit::singleline(&mut self.input_text)
                    .hint_text("Tap to type...")
                    .desired_width(screen_rect.width() * 0.7)
                    .font(egui::FontId::proportional(20.0));
                
                let response = ui.add(text_edit);
                
                if response.clicked() {
                    self.show_keyboard = true;
                }
                
                ui.add_space(20.0);
                
                // Display text
                egui::Frame::none()
                    .fill(egui::Color32::from_rgb(30, 30, 35))
                    .rounding(egui::Rounding::same(12.0))
                    .show(ui, |ui| {
                        ui.add_space(15.0);
                        ui.label(egui::RichText::new("Output:").strong().size(14.0));
                        ui.add_space(5.0);
                        ui.label(egui::RichText::new(&self.input_text).size(18.0).color(egui::Color32::from_rgb(200, 220, 255)));
                        ui.add_space(15.0);
                    });
            });
        });
        
        // Android-style keyboard overlay
        if self.show_keyboard {
            let screen_height = screen_rect.height();
            let keyboard_height = screen_height * 0.45;
            
            egui::Window::new("")
                .collapsible(false)
                .resizable(false)
                .title_bar(false)
                .frame(egui::Frame::none().fill(egui::Color32::from_rgb(30, 30, 35)))
                .anchor(egui::Align2::CENTER_BOTTOM, [0.0, 0.0])
                .fixed_size([screen_rect.width(), keyboard_height])
                .show(ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.add_space(8.0);
                        
                        // Bar indikator
                        ui.horizontal(|ui| {
                            ui.add_space(screen_rect.width() * 0.4);
                            let handle = egui::Button::new("")
                                .fill(egui::Color32::from_rgb(80, 80, 90))
                                .rounding(egui::Rounding::same(5.0));
                            ui.add_sized(egui::vec2(40.0, 4.0), handle);
                        });
                        
                        ui.add_space(12.0);
                        
                        // Baris 1
                        ui.horizontal(|ui| {
                            for key in ["Q", "W", "E", "R", "T", "Y", "U", "I", "O", "P"] {
                                let btn = egui::Button::new(key)
                                    .fill(egui::Color32::from_rgb(50, 50, 55))
                                    .rounding(egui::Rounding::same(8.0));
                                if ui.add_sized(egui::vec2(35.0, 48.0), btn).clicked() {
                                    self.input_text.push_str(key);
                                }
                                ui.add_space(2.0);
                            }
                        });
                        ui.add_space(6.0);
                        
                        // Baris 2
                        ui.horizontal(|ui| {
                            ui.add_space(12.0);
                            for key in ["A", "S", "D", "F", "G", "H", "J", "K", "L"] {
                                let btn = egui::Button::new(key)
                                    .fill(egui::Color32::from_rgb(50, 50, 55))
                                    .rounding(egui::Rounding::same(8.0));
                                if ui.add_sized(egui::vec2(35.0, 48.0), btn).clicked() {
                                    self.input_text.push_str(key);
                                }
                                ui.add_space(2.0);
                            }
                            ui.add_space(12.0);
                        });
                        ui.add_space(6.0);
                        
                        // Baris 3
                        ui.horizontal(|ui| {
                            ui.add_space(18.0);
                            for key in ["Z", "X", "C", "V", "B", "N", "M"] {
                                let btn = egui::Button::new(key)
                                    .fill(egui::Color32::from_rgb(50, 50, 55))
                                    .rounding(egui::Rounding::same(8.0));
                                if ui.add_sized(egui::vec2(35.0, 48.0), btn).clicked() {
                                    self.input_text.push_str(key);
                                }
                                ui.add_space(2.0);
                            }
                            ui.add_space(18.0);
                        });
                        ui.add_space(6.0);
                        
                        // Baris bawah
                        ui.horizontal(|ui| {
                            // Shift
                            let shift_btn = egui::Button::new("⇧")
                                .fill(egui::Color32::from_rgb(50, 50, 55))
                                .rounding(egui::Rounding::same(8.0));
                            if ui.add_sized(egui::vec2(45.0, 48.0), shift_btn).clicked() {
                                // Caps lock sederhana
                                let last_char = self.input_text.chars().last();
                                if let Some(c) = last_char {
                                    if c.is_lowercase() {
                                        let upper = c.to_uppercase().to_string();
                                        self.input_text.pop();
                                        self.input_text.push_str(&upper);
                                    }
                                }
                            }
                            
                            ui.add_space(4.0);
                            
                            // Space
                            let space_btn = egui::Button::new("Space")
                                .fill(egui::Color32::from_rgb(50, 50, 55))
                                .rounding(egui::Rounding::same(8.0));
                            if ui.add_sized(egui::vec2(180.0, 48.0), space_btn).clicked() {
                                self.input_text.push(' ');
                            }
                            
                            ui.add_space(4.0);
                            
                            // Backspace
                            let back_btn = egui::Button::new("⌫")
                                .fill(egui::Color32::from_rgb(50, 50, 55))
                                .rounding(egui::Rounding::same(8.0));
                            if ui.add_sized(egui::vec2(45.0, 48.0), back_btn).clicked() {
                                self.input_text.pop();
                            }
                        });
                        
                        ui.add_space(8.0);
                        
                        // Tombol close
                        let close_btn = egui::Button::new("Done")
                            .fill(egui::Color32::from_rgb(34, 197, 94))
                            .rounding(egui::Rounding::same(20.0));
                        if ui.add_sized(egui::vec2(80.0, 40.0), close_btn).clicked() {
                            self.show_keyboard = false;
                        }
                    });
                });
        }
    }
}
