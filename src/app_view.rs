use eframe::egui;
use crate::app_logic::AppState;
use crate::app_logic::Connection;

pub fn render_ui(ctx: &egui::Context, state: &mut AppState) {
    egui::TopBottomPanel::top("tp")
        .frame(egui::Frame::none().fill(egui::Color32::from_rgb(15, 15, 15)))
        .show(ctx, |ui| {
        ui.add_space(55.0);
        ui.horizontal(|ui| {
            ui.heading(egui::RichText::new("VUZT GRAPH").color(egui::Color32::WHITE).strong());
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("ADD").clicked() { state.add_agent(); }
                if ui.button(if state.is_running { "STOP" } else { "RUN" }).clicked() { state.is_running = !state.is_running;}
                
                // Perbaikan: Warna tombol DEL diset di dalam Button::new
                let del_btn = egui::Button::new(egui::RichText::new("DEL").color(egui::Color32::WHITE))
                    .fill(egui::Color32::from_rgb(180, 0, 0));
                if ui.add(del_btn).clicked() { state.delete_selected(); }
            });
        });
        ui.add_space(10.0);
    });

    egui::CentralPanel::default()
        .frame(egui::Frame::none().fill(egui::Color32::from_rgb(25, 25, 25)))
        .show(ctx, |ui| {
        let painter = ui.painter();
        let time = ctx.input(|i| i.time);

        // --- 1. Connections ---
        for conn in &state.connections {
            if let (Some(a), Some(b)) = (state.agents.get(conn.from), state.agents.get(conn.to)) {
                let start = a.pos + egui::vec2(100.0, 20.0);
                let end = b.pos + egui::vec2(0.0, 20.0);
                painter.line_segment([start, end], egui::Stroke::new(1.2, egui::Color32::from_gray(100)));

                if state.is_running {
                    let t = (time * 0.6 % 1.0) as f32;
                    painter.circle_filled(start + (end - start) * t, 4.0, egui::Color32::YELLOW);
                    ctx.request_repaint();
                }
            }
        }

        // --- 2. Agents Interaction ---
        let mut spawn_target = None;

        for i in 0..state.agents.len() {
            let id = egui::Id::new("ag").with(i);
            let agent = &mut state.agents[i];
            
            let rect = egui::Rect::from_min_size(agent.pos, egui::vec2(100.0, 40.0));
            let plus_rect = egui::Rect::from_min_size(agent.pos + egui::vec2(105.0, 5.0), egui::vec2(30.0, 30.0));

            // Logic Drag & Click Agent Utama
            let resp = ui.interact(rect, id, egui::Sense::click_and_drag());
            if resp.dragged() { agent.pos += resp.drag_delta(); }
            if resp.clicked() { state.selected_agent = Some(i); state.show_kb = true; }

            // Gambar Box Agent
            let stroke = if state.selected_agent == Some(i) { egui::Stroke::new(2.0, egui::Color32::WHITE) } else { egui::Stroke::NONE };
            painter.rect(rect, 8.0, agent.color, stroke);
            painter.text(rect.center(), egui::Align2::CENTER_CENTER, &agent.name, egui::FontId::proportional(14.0), egui::Color32::BLACK);

            // Tombol Plus (+) di Kanan
            let plus_id = id.with("plus");
            let plus_resp = ui.interact(plus_rect, plus_id, egui::Sense::click());
            
            painter.rect_filled(plus_rect, egui::Rounding::same(5.0), egui::Color32::from_rgb(60, 60, 60));
            painter.text(plus_rect.center(), egui::Align2::CENTER_CENTER, "+", egui::FontId::proportional(20.0), egui::Color32::GREEN);

            if plus_resp.clicked() {
                spawn_target = Some(i);
            }
        }

        if let Some(parent_idx) = spawn_target {
            state.spawn_child(parent_idx);
        }

        // Help UI Link
        if state.selected_agent.is_some() && state.link_source.is_none() {
            if ui.put(egui::Rect::from_min_size(egui::pos2(20.0, 130.0), egui::vec2(80.0, 30.0)), egui::Button::new("LINK")).clicked() {
                state.link_source = state.selected_agent;
            }
        }
    });
}
