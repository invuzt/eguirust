use eframe::egui;
use crate::app_logic::AppState;

pub fn render_keyboard(ui: &mut egui::Ui, state: &mut AppState) {
    // Hanya render jika ada node yang dipilih
    if let Some(idx) = state.selected_node_idx {
        if let Some(node) = state.nodes.get_mut(idx) {
            ui.vertical_centered(|ui| {
                ui.label(egui::RichText::new(format!("Editing: {}", node.label)).color(egui::Color32::DEBUG_COLOR));
                
                ui.group(|ui| {
                    let rows = [
                        vec!["1","2","3","4","5","6","7","8","9","0"],
                        vec!["Q","W","E","R","T","Y","U","I","O","P"],
                        vec!["A","S","D","F","G","H","J","K","L",""],
                    ];

                    for row in rows {
                        ui.horizontal(|ui| {
                            for key in row {
                                if ui.add_sized([30.0, 35.0], egui::Button::new(key)).clicked() {
                                    node.label.push_str(key);
                                }
                            }
                        });
                    }

                    ui.horizontal(|ui| {
                        if ui.button("BACKSPACE").clicked() { node.label.pop(); }
                        if ui.button("DONE").clicked() { state.show_kb = false; }
                    });
                });
            });
        }
    }
}
