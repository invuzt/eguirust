use eframe::egui;
use crate::app_logic::AppState;

pub fn render_keyboard(ui: &mut egui::Ui, state: &mut AppState) {
    let btn_size = egui::vec2(30.0, 40.0);
    
    ui.vertical_centered(|ui| {
        ui.add_space(5.0);
        if let Some(idx) = state.selected_node_idx {
            if let Some(node) = state.nodes.get_mut(idx) {
                ui.label(egui::RichText::new(format!("EDIT: {}", node.label)).strong());
                
                ui.group(|ui| {
                    ui.style_mut().spacing.item_spacing = egui::vec2(4.0, 4.0);
                    let rows = [
                        vec!["1","2","3","4","5","6","7","8","9","0"],
                        vec!["Q","W","E","R","T","Y","U","I","O","P"],
                        vec!["A","S","D","F","G","H","J","K","L","@"],
                        vec!["Z","X","C","V","B","N","M",".","_","-"],
                    ];

                    for row in rows {
                        ui.horizontal(|ui| {
                            for key in row {
                                if ui.add_sized(btn_size, egui::Button::new(key)).clicked() {
                                    node.label.push_str(key);
                                }
                            }
                        });
                    }

                    ui.add_space(5.0);
                    ui.horizontal(|ui| {
                        if ui.add_sized([80.0, 35.0], egui::Button::new("HAPUS")).clicked() {
                            node.label.pop();
                        }
                        if ui.add_sized([80.0, 35.0], egui::Button::new("TUTUP")).clicked() {
                            state.show_kb = false;
                        }
                    });
                });
            }
        }
    });
}
