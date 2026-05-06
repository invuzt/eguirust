use eframe::egui;
use crate::app_logic::AppState;

pub fn render_ui(ctx: &egui::Context, state: &mut AppState) {
    // 1. Panel atas sebagai "Status Bar Area" (Gelap)
    egui::TopBottomPanel::top("status_bar")
        .frame(egui::Frame::none().fill(egui::Color32::from_rgb(10, 10, 10)))
        .show(ctx, |ui| {
            ui.add_space(35.0); // Memberi ruang agar tidak kena jam/notif
        });

    egui::CentralPanel::default().show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.heading(egui::RichText::new("VUZT").color(egui::Color32::WHITE));
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("➕ NODE").clicked() {
                    state.add_node();
                }
            });
        });
        ui.separator();

        // Render Nodes di Canvas
        for i in 0..state.nodes.len() {
            let label = &state.nodes[i].label;
            let pos = state.nodes[i].pos;

            let resp = ui.put(
                egui::Rect::from_min_size(pos, egui::vec2(85.0, 35.0)),
                egui::Button::new(label)
                    .fill(egui::Color32::from_rgb(45, 45, 45))
                    .rounding(8.0)
            );

            if resp.clicked() {
                state.selected_node_idx = Some(i);
                state.show_kb = true;
            }
        }
    });
}
