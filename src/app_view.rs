use eframe::egui;
use crate::app_logic::AppState;

pub fn render_ui(ctx: &egui::Context, state: &mut AppState) {
    // 1. Status Bar Area (Padding Extra Safe)
    egui::TopBottomPanel::top("status_bar")
        .frame(egui::Frame::none().fill(egui::Color32::from_rgb(10, 10, 10)))
        .show(ctx, |ui| {
            ui.add_space(50.0); 
        });

    egui::CentralPanel::default().show(ctx, |ui| {
        // Toolbar Atas
        ui.horizontal(|ui| {
            ui.heading(egui::RichText::new("VUZT").color(egui::Color32::WHITE).strong());
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("ADD NODE").clicked() {
                    state.add_node();
                }
            });
        });
        ui.separator();

        // 2. Render & Drag Logic
        for i in 0..state.nodes.len() {
            let node_id = egui::Id::new("node_drag").with(i);
            
            // Ambil data node secara terpisah untuk menghindari borrow conflict
            let mut current_pos = state.nodes[i].pos;
            let current_label = state.nodes[i].label.clone();
            
            let node_rect = egui::Rect::from_min_size(current_pos, egui::vec2(100.0, 40.0));
            let response = ui.interact(node_rect, node_id, egui::Sense::drag());

            if response.dragged() {
                current_pos += response.drag_delta();
                // Update posisi asli di state
                state.nodes[i].pos = current_pos;
            }

            // Gambar Node (Warna Biru Neon agar kontras)
            let fill_color = if response.dragged() {
                egui::Color32::from_rgb(0, 220, 255) // Saat ditarik
            } else {
                egui::Color32::from_rgb(0, 100, 255) // Diam
            };

            ui.painter().rect_filled(node_rect, egui::Rounding::same(8.0), fill_color);
            ui.painter().text(
                node_rect.center(),
                egui::Align2::CENTER_CENTER,
                current_label,
                egui::FontId::proportional(14.0),
                egui::Color32::WHITE,
            );

            // Jika diklik (bukan drag), buka keyboard
            if response.clicked() {
                state.selected_node_idx = Some(i);
                state.show_kb = true;
            }
        }
    });
}
