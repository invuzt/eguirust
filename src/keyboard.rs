use eframe::egui;
use crate::app_logic::AppState;

pub fn render_keyboard(ui: &mut egui::Ui, state: &mut AppState) {
    let btn_size = egui::vec2(32.0, 42.0);
    
    ui.vertical_centered(|ui| {
        ui.add_space(8.0);
        if let Some(idx) = state.selected_node_idx {
            if let Some(node) = state.nodes.get_mut(idx) {
                ui.label(format!("RENAME: {}", node.label));
                
                ui.group(|ui| {
                    let rows = [
                        vec!["1","2","3","4","5","6","7","8","9","0"],
                        vec!["Q","W","E","R","T","Y","U","I","O","P"],
                        vec!["A","S","D","F","G","H","J","K","L","M"],
                        vec!["Z","X","C","V","B","N","."],
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

                    ui.horizontal(|ui| {
                        if ui.button("DELETE").clicked() { node.label.pop(); }
                        if ui.button("CLOSE").clicked() { state.show_kb = false; }
                    });
                });
            }
        }
    });
}
