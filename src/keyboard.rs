use eframe::egui;
use crate::app_logic::{AppState, ActiveField};

pub fn render_keyboard(ui: &mut egui::Ui, state: &mut AppState) {
    ui.vertical_centered(|ui| {
        ui.add_space(5.0);
        
        // Show which field is active
        let active_name = match state.active_field {
            ActiveField::Name => "✏️ NAME ACTIVE",
            ActiveField::Description => "📝 DESCRIPTION ACTIVE",
            ActiveField::None => "SELECT A FIELD",
        };
        ui.label(active_name);
        ui.separator();
        
        // Baris 1
        ui.horizontal(|ui| {
            for key in &["1","2","3","4","5","6","7","8","9","0"] {
                if ui.add_sized([32.0, 38.0], egui::Button::new(*key)).clicked() {
                    state.keyboard_add_char(key.chars().next().unwrap());
                }
            }
        });
        
        // Baris 2
        ui.horizontal(|ui| {
            for key in &["Q","W","E","R","T","Y","U","I","O","P"] {
                if ui.add_sized([32.0, 38.0], egui::Button::new(*key)).clicked() {
                    state.keyboard_add_char(key.chars().next().unwrap());
                }
            }
        });
        
        // Baris 3
        ui.horizontal(|ui| {
            for key in &["A","S","D","F","G","H","J","K","L"] {
                if ui.add_sized([32.0, 38.0], egui::Button::new(*key)).clicked() {
                    state.keyboard_add_char(key.chars().next().unwrap());
                }
            }
        });
        
        // Baris 4
        ui.horizontal(|ui| {
            for key in &["Z","X","C","V","B","N","M"] {
                if ui.add_sized([32.0, 38.0], egui::Button::new(*key)).clicked() {
                    state.keyboard_add_char(key.chars().next().unwrap());
                }
            }
        });
        
        // Baris 5 - Special keys
        ui.horizontal(|ui| {
            if ui.add_sized([55.0, 38.0], egui::Button::new("SPACE")).clicked() {
                state.keyboard_add_space();
            }
            
            if ui.add_sized([50.0, 38.0], egui::Button::new("⌫")).clicked() {
                state.keyboard_backspace();
            }
            
            if ui.add_sized([70.0, 38.0], egui::Button::new("DEL WORD")).clicked() {
                state.keyboard_delete_word();
            }
            
            if ui.add_sized([50.0, 38.0], egui::Button::new("CLOSE")).clicked() {
                state.show_kb = false;
                state.active_field = ActiveField::None;
            }
        });
        
        ui.add_space(8.0);
    });
}
