use eframe::egui;
use crate::app_logic::AppState;

pub fn render_keyboard(ui: &mut egui::Ui, state: &mut AppState) {
    let btn_size = egui::vec2(32.0, 42.0);
    
    ui.vertical_centered(|ui| {
        ui.add_space(8.0);
        // Sinkronisasi: Menggunakan selected_agent dan agents (bukan node)
        if let Some(idx) = state.selected_agent {
            if let Some(agent) = state.agents.get_mut(idx) {
                ui.label(egui::RichText::new(format!("AGENT NAME: {}", agent.name)).strong());
                
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
                                    agent.name.push_str(key);
                                }
                            }
                        });
                    }

                    ui.add_space(10.0);
                    ui.horizontal(|ui| {
                        if ui.add_sized([100.0, 35.0], egui::Button::new("HAPUS")).clicked() {
                            agent.name.pop();
                        }
                        if ui.add_sized([100.0, 35.0], egui::Button::new("CLOSE")).clicked() {
                            state.show_kb = false;
                        }
                    });
                });
            }
        }
    });
}
