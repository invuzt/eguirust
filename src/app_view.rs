use eframe::egui;
use crate::app_logic::AppState;

pub fn render_ui(ctx: &egui::Context, state: &mut AppState) {
    egui::TopBottomPanel::top("tp").frame(egui::Frame::none().fill(egui::Color32::BLACK)).show(ctx, |ui| {
        ui.add_space(55.0);
    });

    egui::CentralPanel::default().show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.heading(egui::RichText::new("VUZT AGENTIC").color(egui::Color32::WHITE).strong());
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("ADD AGENT").clicked() { state.add_agent(); }
                let btn_text = if state.is_running { "STOP SIM" } else { "START SIM" };
                if ui.button(btn_text).clicked() { state.is_running = !state.is_running;}
            });
        });
        ui.separator();

        let painter = ui.painter();
        let time = ctx.input(|i| i.time);

        // --- 1. Render Connections & Animated Messages ---
        for conn in &state.connections {
            if let (Some(a), Some(b)) = (state.agents.get(conn.from), state.agents.get(conn.to)) {
                let start = a.pos + egui::vec2(50.0, 20.0);
                let end = b.pos + egui::vec2(50.0, 20.0);
                
                // Garis Abu-abu
                painter.line_segment([start, end], egui::Stroke::new(1.0, egui::Color32::from_gray(60)));

                if state.is_running {
                    let t = (time * 0.7 % 1.0) as f32; // Progress 0.0 ke 1.0
                    let dot_pos = start + (end - start) * t;

                    // Gambar Titik Data (Kuning)
                    painter.circle_filled(dot_pos, 4.5, egui::Color32::from_rgb(255, 220, 0));

                    // Tampilkan Pesan jika titik sudah jalan lebih dari setengah
                    if t > 0.3 && t < 0.9 {
                        painter.text(
                            dot_pos + egui::vec2(0.0, -15.0),
                            egui::Align2::CENTER_BOTTOM,
                            &conn.message,
                            egui::FontId::proportional(11.0),
                            egui::Color32::LIGHT_YELLOW,
                        );
                    }
                    ctx.request_repaint(); 
                }
            }
        }

        // --- 2. Render Agents ---
        for i in 0..state.agents.len() {
            let id = egui::Id::new("ag").with(i);
            let mut pos = state.agents[i].pos;
            let rect = egui::Rect::from_min_size(pos, egui::vec2(100.0, 40.0));
            let resp = ui.interact(rect, id, egui::Sense::drag());

            if resp.dragged() {
                pos += resp.drag_delta();
                state.agents[i].pos = pos;
            }

            // Box Agent
            painter.rect_filled(rect, egui::Rounding::same(6.0), state.agents[i].color);
            painter.text(rect.center(), egui::Align2::CENTER_CENTER, &state.agents[i].name, egui::FontId::proportional(14.0), egui::Color32::BLACK);

            if resp.clicked() {
                state.selected_agent = Some(i);
                state.show_kb = true;
            }
        }
    });
}
