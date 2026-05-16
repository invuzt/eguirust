use eframe::egui;

pub struct MyApp {
    text: String,
    show_keyboard: bool,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            text: String::new(),
            show_keyboard: false,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let screen_rect = ctx.input(|i| i.screen_rect());
        let screen_height = screen_rect.height();
        
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(screen_height * 0.15);
                
                ui.heading("Vuzt Simple Input");
                ui.add_space(20.0);
                
                ui.label("Input Teks:");
                ui.add_space(10.0);
                
                let text_edit = egui::TextEdit::singleline(&mut self.text)
                    .hint_text("Ketik sesuatu...")
                    .desired_width(300.0)
                    .font(egui::FontId::proportional(18.0));
                
                ui.add(text_edit);
                
                ui.add_space(20.0);
                
                if !self.text.is_empty() {
                    ui.label(format!("Output: {}", self.text));
                }
                
                ui.add_space(30.0);
                
                if ui.button("Clear").clicked() {
                    self.text.clear();
                }
            });
        });
        
        // Custom keyboard sederhana
        if self.show_keyboard {
            let screen_width = screen_rect.width();
            let key_width = screen_width / 10.0;
            
            egui::Window::new("Keyboard")
                .collapsible(false)
                .resizable(false)
                .title_bar(true)
                .anchor(egui::Align2::CENTER_BOTTOM, [0.0, -10.0])
                .show(ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        let rows = [
                            vec!["1","2","3","4","5","6","7","8","9","0"],
                            vec!["Q","W","E","R","T","Y","U","I","O","P"],
                            vec!["A","S","D","F","G","H","J","K","L"],
                            vec!["Z","X","C","V","B","N","M"],
                            vec![" ", "-", "_", ".", "@"],
                        ];
                        
                        for row in rows {
                            ui.horizontal(|ui| {
                                for key in row {
                                    let btn = egui::Button::new(key)
                                        .fill(egui::Color32::from_rgb(55, 55, 65))
                                        .rounding(egui::Rounding::same(8.0));
                                    
                                    if ui.add_sized(egui::vec2(key_width, 48.0), btn).clicked() {
                                        if key == " " {
                                            self.text.push(' ');
                                        } else {
                                            self.text.push_str(key);
                                        }
                                    }
                                }
                            });
                            ui.add_space(6.0);
                        }
                        
                        ui.add_space(12.0);
                        ui.horizontal(|ui| {
                            if ui.button("Hapus").clicked() {
                                self.text.pop();
                            }
                            ui.add_space(20.0);
                            if ui.button("Tutup").clicked() {
                                self.show_keyboard = false;
                            }
                        });
                    });
                });
        }
    }
}
