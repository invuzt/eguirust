use eframe::egui;
use crate::app_logic::AppState;

pub fn render_keyboard(ui: &mut egui::Ui, state: &mut AppState) {
    let screen_rect = ui.ctx().input(|i| i.screen_rect());
    let bottom_margin = screen_rect.height() * 0.35;
    
    ui.add_space(bottom_margin);
    ui.vertical_centered(|ui| {
        if let Some(idx) = state.selected_node {
            if let Some(node) = state.nodes.get_mut(idx) {
                egui::Frame::none()
                    .fill(egui::Color32::from_rgba_premultiplied(25,25,35,250))
                    .rounding(egui::Rounding::same(20.0))
                    .show(ui, |ui| {
                        ui.add_space(16.0);
                        ui.label(egui::RichText::new(format!("✏️ {}", node.name)).size(20.0).color(egui::Color32::WHITE));
                        ui.add_space(16.0);
                        
                        let rows = [
                            vec!["1","2","3","4","5","6","7","8","9","0"],
                            vec!["Q","W","E","R","T","Y","U","I","O","P"],
                            vec!["A","S","D","F","G","H","J","K","L"],
                            vec!["Z","X","C","V","B","N","M"],
                            vec![".", "-", "_", " "],
                        ];
                        
                        for row in rows {
                            ui.horizontal(|ui| {
                                for key in row {
                                    let btn = egui::Button::new(egui::RichText::new(key).size(20.0))
                                        .fill(egui::Color32::from_rgb(55,55,65))
                                        .rounding(egui::Rounding::same(40.0));
                                    if ui.add_sized(egui::vec2(55.0, 55.0), btn).clicked() {
                                        node.name.push_str(key);
                                    }
                                }
                            });
                            ui.add_space(10.0);
                        }
                        
                        ui.add_space(16.0);
                        ui.horizontal(|ui| {
                            let del_btn = egui::Button::new(egui::RichText::new("⌫ HAPUS").size(16.0))
                                .fill(egui::Color32::from_rgb(220,38,38))
                                .rounding(egui::Rounding::same(30.0));
                            if ui.add_sized(egui::vec2(130.0, 50.0), del_btn).clicked() {
                                node.name.pop();
                            }
                            
                            ui.add_space(30.0);
                            
                            let close_btn = egui::Button::new(egui::RichText::new("✓ TUTUP").size(16.0))
                                .fill(egui::Color32::from_rgb(34,197,94))
                                .rounding(egui::Rounding::same(30.0));
                            if ui.add_sized(egui::vec2(130.0, 50.0), close_btn).clicked() {
                                state.show_kb = false;
                            }
                        });
                        ui.add_space(20.0);
                    });
            }
        }
    });
}
