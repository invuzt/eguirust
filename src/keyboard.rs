use eframe::egui;
use crate::app_logic::AppState;

pub fn render_keyboard(ui: &mut egui::Ui, state: &mut AppState) {
    ui.vertical_centered(|ui| {
        ui.add_space(5.0);
        
        // Header keyboard dengan tombol close
        ui.horizontal(|ui| {
            let field_name = match state.selected_field {
                crate::app_logic::FieldType::Name => "EDITING: NAME",
                crate::app_logic::FieldType::Description => "EDITING: DESCRIPTION",
            };
            ui.label(field_name);
            
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("X").clicked() {
                    state.show_kb = false;
                }
            });
        });
        
        ui.add_space(5.0);
        ui.separator();
        
        // Baris 1
        ui.horizontal(|ui| {
            for key in &["1","2","3","4","5","6","7","8","9","0"] {
                if ui.add_sized([32.0, 35.0], egui::Button::new(*key)).clicked() {
                    state.add_char_to_selected(key.chars().next().unwrap());
                }
            }
        });
        
        // Baris 2
        ui.horizontal(|ui| {
            for key in &["Q","W","E","R","T","Y","U","I","O","P"] {
                if ui.add_sized([32.0, 35.0], egui::Button::new(*key)).clicked() {
                    state.add_char_to_selected(key.chars().next().unwrap());
                }
            }
        });
        
        // Baris 3
        ui.horizontal(|ui| {
            for key in &["A","S","D","F","G","H","J","K","L"] {
                if ui.add_sized([32.0, 35.0], egui::Button::new(*key)).clicked() {
                    state.add_char_to_selected(key.chars().next().unwrap());
                }
            }
        });
        
        // Baris 4
        ui.horizontal(|ui| {
            for key in &["Z","X","C","V","B","N","M"] {
                if ui.add_sized([32.0, 35.0], egui::Button::new(*key)).clicked() {
                    state.add_char_to_selected(key.chars().next().unwrap());
                }
            }
        });
        
        // Baris 5 - Special keys
        ui.horizontal(|ui| {
            if ui.add_sized([55.0, 35.0], egui::Button::new("SPACE")).clicked() {
                state.add_space();
            }
            
            if ui.add_sized([50.0, 35.0], egui::Button::new("BACK")).clicked() {
                state.delete_last_char();
            }
            
            if ui.add_sized([70.0, 35.0], egui::Button::new("DEL WORD")).clicked() {
                state.delete_last_word();
            }
        });
        
        ui.add_space(10.0);
    });
}
