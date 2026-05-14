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
        let screen_width = screen_rect.width();
        let screen_height = screen_rect.height();
        
        // Responsive sizes
        let key_width = screen_width / 11.0;
        let key_height = key_width * 1.2;
        let space_width = screen_width / 2.5;
        
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(screen_height * 0.08);
                ui.heading(egui::RichText::new("Android Style Keyboard").size(24.0).color(egui::Color32::WHITE));
                ui.add_space(screen_height * 0.03);
                
                // Input field
                let text_edit = egui::TextEdit::singleline(&mut self.input_text)
                    .hint_text("Tap to type...")
                    .desired_width(screen_width * 0.85)
                    .font(egui::FontId::proportional(18.0));
                
                let response = ui.add(text_edit);
                
                if response.clicked() {
                    self.show_keyboard = true;
                }
                
                ui.add_space(screen_height * 0.02);
                
                // Output display
                egui::Frame::none()
                    .fill(egui::Color32::from_rgb(30, 30, 35))
                    .rounding(egui::Rounding::same(12.0))
                    .show(ui, |ui| {
                        ui.add_space(12.0);
                        ui.label(egui::RichText::new("Output:").strong().size(14.0));
                        ui.add_space(5.0);
                        ui.label(egui::RichText::new(&self.input_text).size(16.0).color(egui::Color32::from_rgb(200, 220, 255)));
                        ui.add_space(12.0);
                    });
            });
        });
        
        // Responsive Android keyboard
        if self.show_keyboard {
            let keyboard_height = screen_height * 0.42;
            
            egui::Window::new("")
                .collapsible(false)
                .resizable(false)
                .title_bar(false)
                .frame(egui::Frame::none().fill(egui::Color32::from_rgb(30, 30, 35)))
                .anchor(egui::Align2::CENTER_BOTTOM, [0.0, 0.0])
                .fixed_size([screen_width, keyboard_height])
                .show(ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.add_space(6.0);
                        
                        // Handle bar
                        ui.horizontal(|ui| {
                            ui.add_space(screen_width * 0.42);
                            let handle = egui::Button::new("")
                                .fill(egui::Color32::from_rgb(80, 80, 90))
                                .rounding(egui::Rounding::same(4.0));
                            ui.add_sized(egui::vec2(40.0, 4.0), handle);
                        });
                        
                        ui.add_space(10.0);
                        
                        // Row 1
                        ui.horizontal(|ui| {
                            ui.add_space(screen_width * 0.02);
                            for key in ["Q", "W", "E", "R", "T", "Y", "U", "I", "O", "P"] {
                                let btn = egui::Button::new(key)
                                    .fill(egui::Color32::from_rgb(55, 55, 60))
                                    .rounding(egui::Rounding::same(8.0));
                                if ui.add_sized(egui::vec2(key_width, key_height), btn).clicked() {
                                    self.input_text.push_str(key);
                                }
                                ui.add_space(2.0);
                            }
                            ui.add_space(screen_width * 0.02);
                        });
                        ui.add_space(6.0);
                        
                        // Row 2
                        ui.horizontal(|ui| {
                            ui.add_space(screen_width * 0.08);
                            for key in ["A", "S", "D", "F", "G", "H", "J", "K", "L"] {
                                let btn = egui::Button::new(key)
                                    .fill(egui::Color32::from_rgb(55, 55, 60))
                                    .rounding(egui::Rounding::same(8.0));
                                if ui.add_sized(egui::vec2(key_width, key_height), btn).clicked() {
                                    self.input_text.push_str(key);
                                }
                                ui.add_space(2.0);
                            }
                            ui.add_space(screen_width * 0.08);
                        });
                        ui.add_space(6.0);
                        
                        // Row 3
                        ui.horizontal(|ui| {
                            ui.add_space(screen_width * 0.12);
                            for key in ["Z", "X", "C", "V", "B", "N", "M"] {
                                let btn = egui::Button::new(key)
                                    .fill(egui::Color32::from_rgb(55, 55, 60))
                                    .rounding(egui::Rounding::same(8.0));
                                if ui.add_sized(egui::vec2(key_width, key_height), btn).clicked() {
                                    self.input_text.push_str(key);
                                }
                                ui.add_space(2.0);
                            }
                            ui.add_space(screen_width * 0.12);
                        });
                        ui.add_space(8.0);
                        
                        // Bottom row
                        ui.horizontal(|ui| {
                            ui.add_space(screen_width * 0.05);
                            
                            // Shift
                            let shift_btn = egui::Button::new("⇧")
                                .fill(egui::Color32::from_rgb(55, 55, 60))
                                .rounding(egui::Rounding::same(8.0));
                            if ui.add_sized(egui::vec2(key_width * 1.2, key_height), shift_btn).clicked() {
                                if let Some(last) = self.input_text.chars().last() {
                                    if last.is_lowercase() {
                                        let upper = last.to_uppercase().to_string();
                                        self.input_text.pop();
                                        self.input_text.push_str(&upper);
                                    }
                                }
                            }
                            
                            ui.add_space(6.0);
                            
                            // Space
                            let space_btn = egui::Button::new("Space")
                                .fill(egui::Color32::from_rgb(55, 55, 60))
                                .rounding(egui::Rounding::same(8.0));
                            if ui.add_sized(egui::vec2(space_width, key_height), space_btn).clicked() {
                                self.input_text.push(' ');
                            }
                            
                            ui.add_space(6.0);
                            
                            // Backspace
                            let back_btn = egui::Button::new("⌫")
                                .fill(egui::Color32::from_rgb(55, 55, 60))
                                .rounding(egui::Rounding::same(8.0));
                            if ui.add_sized(egui::vec2(key_width * 1.2, key_height), back_btn).clicked() {
                                self.input_text.pop();
                            }
                            
                            ui.add_space(screen_width * 0.05);
                        });
                        
                        ui.add_space(10.0);
                        
                        // Done button
                        let done_btn = egui::Button::new("Done")
                            .fill(egui::Color32::from_rgb(34, 197, 94))
                            .rounding(egui::Rounding::same(20.0));
                        if ui.add_sized(egui::vec2(screen_width * 0.25, 40.0), done_btn).clicked() {
                            self.show_keyboard = false;
                        }
                        
                        ui.add_space(8.0);
                    });
                });
        }
    }
}
