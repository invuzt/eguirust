use eframe::egui;
use crate::app_logic::AppState;

pub fn render_ui(ctx: &egui::Context, state: &mut AppState) {
    egui::TopBottomPanel::top("tp").frame(egui::Frame::none().fill(egui::Color32::BLACK)).show(ctx, |ui| {
        ui.add_space(55.0);
    });

    egui::CentralPanel::default().show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.heading("VUZT FLOW");
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("ADD AGENT").clicked() { state.add_agent(); }
                if ui.button(if state.is_running { "STOP" } else { "RUN SIM" }).clicked() {
                    state.is_running = !state.is_running;
                }
            });
        });
        ui.separator();

        let painter = ui.painter();
        let time = ctx.input(|i| i.time); // Ambil waktu sistem untuk animasi

        // --- 1. Gambar Garis dengan Panah & Animasi ---
        for conn in &state.connections {
            if let (Some(a), Some(b)) = (state.agents.get(conn.from), state.agents.get(conn.to)) {
                let start = a.pos + egui::vec2(50.0, 20.0);
                let end = b.pos + egui::vec2(50.0, 20.0);
                
                // Gambar Garis Utama
                painter.line_segment([start, end], egui::Stroke::new(1.5, egui::Color32::DARK_GRAY));

                // Hitung Arah Panah
                let vec = end - start;
                let dir = vec / vec.length();
                let arrow_pos = end - dir * 10.0;
                let side = egui::vec2(-dir.y, dir.x) * 5.0;
                
                painter.line_segment([end, arrow_pos + side], egui::Stroke::new(2.0, egui::Color32::GRAY));
                painter.line_segment([end, arrow_pos - side], egui::Stroke::new(2.0, egui::Color32::GRAY));

                // Simulasi Titik Data Bergerak
                if state.is_running {
                    let t = (time * 0.8 % 1.0) as f32; // Kecepatan gerak
                    let dot_pos = start + vec * t;
                    painter.circle_filled(dot_pos, 4.0, egui::Color32::YELLOW);
                    ctx.request_repaint(); // Paksa render ulang agar animasi jalan
                }
            }
        }

        // --- 2. Render Agent (Drag & Interaction) ---
        for i in 0..state.agents.len() {
            let id = egui::Id::new("ag").with(i);
            let mut pos = state.agents[i].pos;
            let rect = egui::Rect::from_min_size(pos, egui::vec2(100.0, 40.0));
            let resp = ui.interact(rect, id, egui::Sense::drag());

            if resp.dragged() {
                pos += resp.drag_delta();
                state.agents[i].pos = pos;
            }

            painter.rect_filled(rect, egui::Rounding::same(8.0), state.agents[i].color);
            painter.text(rect.center(), egui::Align2::CENTER_CENTER, &state.agents[i].name, egui::FontId::proportional(14.0), egui::Color32::BLACK);

            if resp.clicked() {
                state.selected_agent = Some(i);
                state.show_kb = true;
            }
        }
    });
}
