use eframe::egui;
use crate::app_logic::AppState;

pub fn render_ui(ctx: &egui::Context, state: &mut AppState) {
    // 1. Status Bar Area (Padding ditambahkan jadi 45.0)
    egui::TopBottomPanel::top("status_bar")
        .frame(egui::Frame::none().fill(egui::Color32::from_rgb(15, 15, 15)))
        .show(ctx, |ui| {
            ui.add_space(45.0); 
        });

    egui::CentralPanel::default().show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.heading(egui::RichText::new("VUZT").color(egui::Color32::WHITE).strong());
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("ADD NODE").clicked() {
                    state.add_node();
                }
            });
        });
        ui.separator();

        // 2. Render & Drag Logic untuk Nodes
        let mut node_to_delete = None;
        
        for i in 0..state.nodes.len() {
            let node = &mut state.nodes[i];
            let node_id = egui::Id::new("node").with(i);
            
            // Membuat area interaksi untuk node
            let mut node_rect = egui::Rect::from_min_size(node.pos, egui::vec2(100.0, 40.0));
            
            // Cek interaksi (Drag)
            let response = ui.interact(node_rect, node_id, egui::Sense::drag());
            
            // Jika digeser, update posisi node berdasarkan delta gerakan jari
            if response.dragged() {
                node.pos += response.drag_delta();
            }

            // Gambar Node (Warna Biru Cerah agar kelihatan)
            ui.painter().rect_filled(
                node_rect, 
                egui::Rounding::same(10.0), 
                if response.dragged() { egui::Color32::from_rgb(0, 200, 255) } else { egui::Color32::from_rgb(0, 120, 255) }
            );

            // Teks di dalam Node
            ui.painter().text(
                node_rect.center(),
                egui::Align2::CENTER_CENTER,
                &node.label,
                egui::FontId::proportional(16.0),
                egui::Color32::WHITE,
            );

            // Klik biasa untuk buka keyboard
            if response.clicked() {
                state.selected_node_idx = Some(i);
                state.show_kb = true;
            }
        }
    });
}
