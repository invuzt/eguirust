use eframe::egui;
use crate::app_logic::AppState;

pub fn render_ui(ctx: &egui::Context, state: &mut AppState) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading(&state.app_name);
        ui.separator();

        if ui.button("➕ SPAWN NEW NODE").clicked() {
            state.add_node();
        }

        ui.add_space(20.0);

        // Area Canvas (Simulasi)
        for i in 0..state.nodes.len() {
            let label = state.nodes[i].label.clone();
            let pos = state.nodes[i].pos;

            // Render node sebagai tombol di posisi tertentu
            ui.put(egui::Rect::from_min_size(pos, egui::vec2(80.0, 30.0)), 
                egui::Button::new(&label).rounding(5.0)
            ).clicked().then(|| {
                state.selected_node_idx = Some(i);
                state.show_kb = true;
            });
        }
    });
}
