use eframe::egui;
use crate::app_logic::AppState;

pub fn render_ui(ctx: &egui::Context, state: &mut AppState) {
    // Top Panel untuk Safe Area
    egui::TopBottomPanel::top("top_p").frame(egui::Frame::none().fill(egui::Color32::BLACK)).show(ctx, |ui| {
        ui.add_space(55.0);
    });

    egui::CentralPanel::default().show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.heading("VUZT AGENT SIM");
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("ADD AGENT").clicked() {
                    state.add_agent();
                }
            });
        });
        ui.separator();

        let painter = ui.painter();

        // --- 1. Gambar Garis (Connections) ---
        for conn in &state.connections {
            if let (Some(a), Some(b)) = (state.agents.get(conn.from), state.agents.get(conn.to)) {
                painter.line_segment(
                    [a.pos + egui::vec2(50.0, 20.0), b.pos + egui::vec2(50.0, 20.0)],
                    egui::Stroke::new(2.0, egui::Color32::GRAY)
                );
            }
        }

        // --- 2. Gambar & Drag Agents ---
        for i in 0..state.agents.len() {
            let agent_id = egui::Id::new("agent").with(i);
            let mut agent_pos = state.agents[i].pos;
            let agent_color = state.agents[i].color;
            let agent_name = state.agents[i].name.clone();

            let rect = egui::Rect::from_min_size(agent_pos, egui::vec2(100.0, 40.0));
            let resp = ui.interact(rect, agent_id, egui::Sense::drag());

            if resp.dragged() {
                agent_pos += resp.drag_delta();
                state.agents[i].pos = agent_pos;
            }

            // Visual Agent
            painter.rect_filled(rect, egui::Rounding::same(12.0), agent_color);
            painter.text(rect.center(), egui::Align2::CENTER_CENTER, agent_name, egui::FontId::proportional(14.0), egui::Color32::BLACK);

            if resp.clicked() {
                state.selected_agent = Some(i);
                state.show_kb = true;
            }
        }
    });
}
