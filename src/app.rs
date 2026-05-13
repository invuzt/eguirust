use eframe::egui;

#[derive(Default)]
pub struct MyApp {
    input_text: String,
    output_text: String,
    buttons: Vec<ButtonWidget>,
    show_keyboard: bool,
}

#[derive(Clone)]
struct ButtonWidget {
    id: String,
    label: String,
    color: egui::Color32,
    action: ButtonAction,
}

#[derive(Clone)]
enum ButtonAction {
    ProcessText(String),
    Clear,
    Reverse,
    Uppercase,
    Lowercase,
    CountChars,
    None,
}

impl MyApp {
    pub fn new() -> Self {
        let mut app = Self::default();
        app.buttons = vec![
            ButtonWidget {
                id: "btn_upper".to_string(),
                label: "🔠 UPPERCASE".to_string(),
                color: egui::Color32::from_rgb(59, 130, 246),
                action: ButtonAction::Uppercase,
            },
            ButtonWidget {
                id: "btn_lower".to_string(),
                label: "🔡 lowercase".to_string(),
                color: egui::Color32::from_rgb(34, 197, 94),
                action: ButtonAction::Lowercase,
            },
            ButtonWidget {
                id: "btn_reverse".to_string(),
                label: "🔄 Reverse".to_string(),
                color: egui::Color32::from_rgb(168, 85, 247),
                action: ButtonAction::Reverse,
            },
            ButtonWidget {
                id: "btn_count".to_string(),
                label: "🔢 Count Chars".to_string(),
                color: egui::Color32::from_rgb(239, 68, 68),
                action: ButtonAction::CountChars,
            },
            ButtonWidget {
                id: "btn_clear".to_string(),
                label: "🗑 Clear".to_string(),
                color: egui::Color32::from_rgb(107, 114, 128),
                action: ButtonAction::Clear,
            },
        ];
        app
    }
    
    fn process_action(&mut self, action: &ButtonAction) {
        match action {
            ButtonAction::ProcessText(cmd) => {
                self.output_text = format!("Processing: {} -> {}", cmd, self.input_text);
            }
            ButtonAction::Clear => {
                self.input_text.clear();
                self.output_text.clear();
            }
            ButtonAction::Reverse => {
                self.output_text = self.input_text.chars().rev().collect();
            }
            ButtonAction::Uppercase => {
                self.output_text = self.input_text.to_uppercase();
            }
            ButtonAction::Lowercase => {
                self.output_text = self.input_text.to_lowercase();
            }
            ButtonAction::CountChars => {
                self.output_text = format!("Length: {} characters", self.input_text.len());
            }
            ButtonAction::None => {}
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let screen_rect = ctx.input(|i| i.screen_rect());
        let top_margin = screen_rect.height() * 0.08;
        let side_margin = screen_rect.width() * 0.05;
        let btn_width = (screen_rect.width() / 2.5).max(150.0).min(200.0);
        let btn_height = 50.0;
        
        // Custom keyboard toggle
        let show_kb = self.show_keyboard;
        
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(top_margin);
            ui.vertical_centered(|ui| {
                // Title
                ui.heading(egui::RichText::new("📱 VUZT SIMPLE UI").size(24.0).color(egui::Color32::WHITE));
                ui.add_space(30.0);
                
                // Input field with custom keyboard trigger
                ui.horizontal(|ui| {
                    ui.add_space(side_margin);
                    let text_edit = egui::TextEdit::singleline(&mut self.input_text)
                        .hint_text("Tap here to type...")
                        .desired_width(screen_rect.width() * 0.6)
                        .font(egui::FontId::proportional(18.0));
                    
                    let response = ui.add(text_edit);
                    
                    if response.clicked() {
                        self.show_keyboard = true;
                    }
                    
                    ui.add_space(10.0);
                    
                    let kb_btn = egui::Button::new(egui::RichText::new("⌨️").size(24.0))
                        .rounding(egui::Rounding::same(30.0));
                    if ui.add_sized(egui::vec2(55.0, 45.0), kb_btn).clicked() {
                        self.show_keyboard = !self.show_keyboard;
                    }
                    ui.add_space(side_margin);
                });
                
                ui.add_space(20.0);
                
                // Output display
                egui::Frame::none()
                    .fill(egui::Color32::from_rgb(30, 30, 35))
                    .rounding(egui::Rounding::same(12.0))
                    .show(ui, |ui| {
                        ui.add_space(10.0);
                        ui.label(egui::RichText::new("📤 OUTPUT:").strong().size(16.0));
                        ui.add_space(5.0);
                        ui.label(egui::RichText::new(&self.output_text).size(18.0).color(egui::Color32::from_rgb(200, 220, 255)));
                        ui.add_space(10.0);
                    });
                
                ui.add_space(30.0);
                
                // Dynamic buttons - rendered from vec
                ui.horizontal(|ui| {
                    ui.add_space(side_margin);
                    ui.vertical(|ui| {
                        for btn in &mut self.buttons {
                            let button = egui::Button::new(egui::RichText::new(&btn.label).color(egui::Color32::WHITE).size(14.0))
                                .fill(btn.color)
                                .rounding(egui::Rounding::same(30.0));
                            
                            if ui.add_sized(egui::vec2(btn_width, btn_height), button).clicked() {
                                self.process_action(&btn.action);
                            }
                            ui.add_space(12.0);
                        }
                    });
                    ui.add_space(side_margin);
                });
            });
        });
        
        // Custom keyboard overlay
        if show_kb {
            egui::Window::new("⌨️ Keyboard")
                .collapsible(false)
                .resizable(false)
                .title_bar(true)
                .anchor(egui::Align2::CENTER_BOTTOM, [0.0, -50.0])
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
                                    let btn = egui::Button::new(egui::RichText::new(*key).size(24.0))
                                        .fill(egui::Color32::from_rgb(60, 60, 70))
                                        .rounding(egui::Rounding::same(30.0));
                                    
                                    if ui.add_sized(egui::vec2(55.0, 55.0), btn).clicked() {
                                        if *key == " " {
                                            self.input_text.push(' ');
                                        } else if *key == "@" {
                                            self.input_text.push('@');
                                        } else {
                                            self.input_text.push_str(key);
                                        }
                                    }
                                }
                            });
                            ui.add_space(8.0);
                        }
                        
                        ui.add_space(12.0);
                        ui.horizontal(|ui| {
                            let del_btn = egui::Button::new(egui::RichText::new("⌫ HAPUS").size(16.0))
                                .fill(egui::Color32::from_rgb(220, 38, 38))
                                .rounding(egui::Rounding::same(25.0));
                            if ui.add_sized(egui::vec2(130.0, 50.0), del_btn).clicked() {
                                self.input_text.pop();
                            }
                            
                            ui.add_space(20.0);
                            
                            let close_btn = egui::Button::new(egui::RichText::new("✓ TUTUP").size(16.0))
                                .fill(egui::Color32::from_rgb(34, 197, 94))
                                .rounding(egui::Rounding::same(25.0));
                            if ui.add_sized(egui::vec2(130.0, 50.0), close_btn).clicked() {
                                self.show_keyboard = false;
                            }
                        });
                        ui.add_space(20.0);
                    });
                });
        }
    }
}
