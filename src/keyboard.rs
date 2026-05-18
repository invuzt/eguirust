use eframe::egui;
use crate::app_logic::AppState;

pub fn render_keyboard(ui: &mut egui::Ui, state: &mut AppState) {
    // Gunakan layout scroll horizontal untuk keyboard
    egui::ScrollArea::horizontal()
        .auto_shrink([false; 2])
        .show(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(5.0);
                
                // Indikator field yang sedang diedit
                let field_name = match state.selected_field {
                    crate::app_logic::FieldType::Name => "NAME",
                    crate::app_logic::FieldType::Description => "DESCRIPTION",
                };
                ui.label(format!("Editing: {}", field_name));
                ui.add_space(5.0);
                
                // Baris 1
                ui.horizontal(|ui| {
                    for key in &["1","2","3","4","5","6","7","8","9","0"] {
                        if ui.add_sized([36.0, 40.0], egui::Button::new(*key)).clicked() {
                            state.add_char_to_selected(key.chars().next().unwrap());
                        }
                    }
                });
                
                // Baris 2
                ui.horizontal(|ui| {
                    for key in &["Q","W","E","R","T","Y","U","I","O","P"] {
                        if ui.add_sized([36.0, 40.0], egui::Button::new(*key)).clicked() {
                            state.add_char_to_selected(key.chars().next().unwrap());
                        }
                    }
                });
                
                // Baris 3
                ui.horizontal(|ui| {
                    for key in &["A","S","D","F","G","H","J","K","L"] {
                        if ui.add_sized([36.0, 40.0], egui::Button::new(*key)).clicked() {
                            state.add_char_to_selected(key.chars().next().unwrap());
                        }
                    }
                });
                
                // Baris 4
                ui.horizontal(|ui| {
                    for key in &["Z","X","C","V","B","N","M"] {
                        if ui.add_sized([36.0, 40.0], egui::Button::new(*key)).clicked() {
                            state.add_char_to_selected(key.chars().next().unwrap());
                        }
                    }
                });
                
                // Baris 5 - Special keys
                ui.horizontal(|ui| {
                    if ui.add_sized([60.0, 40.0], egui::Button::new("SPACE")).clicked() {
                        state.add_space();
                    }
                    
                    if ui.add_sized([60.0, 40.0], egui::Button::new("BACK")).clicked() {
                        state.delete_last_char();
                    }
                    
                    if ui.add_sized([80.0, 40.0], egui::Button::new("DEL WORD")).clicked() {
                        state.delete_last_word();
                    }
                    
                    if ui.add_sized([60.0, 40.0], egui::Button::new("CLOSE")).clicked() {
                        state.show_kb = false;
                    }
                });
                
                ui.add_space(10.0);
            });
        });
}
