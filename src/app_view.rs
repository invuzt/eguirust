use eframe::egui;
use crate::app_logic::AppState;
use crate::app_logic::Connection;

pub fn render_ui(ctx: &egui::Context, state: &mut AppState) {
    // Top Panel dengan warna gelap solid
    egui::TopBottomPanel::top("tp").frame(egui::Frame::none().fill(egui::Color32::from_rgb(15, 15, 15))).show(ctx, |ui| {
        ui.add_space(55.0);
        ui.horizontal(|ui| {
            ui.heading(egui::RichText::new("VUZT GRAPH").color(egui::Color32::WHITE));
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("ADD").clicked() { state.add_agent(); }
                if ui.button(if state.is_running { "STOP" } else { "RUN" }).clicked() { state.is_running = !state.is_running;}
                if ui.button("DEL").fill(egui::Color32::RED).clicked() { state.delete_selected(); }
            });
        });
        ui.add_space(10.0);
    });

    egui::CentralPanel::default().frame(egui::Frame::none().fill(egui::Color32::from_rgb(25, 25, 25))).show(ctx, |ui| {
        let painter = ui.painter();
        let time = ctx.input(|i| i.time);

        // --- 1. Draw Connections ---
        for conn in &state.connections {
            if let (Some(a), Some(b)) = (state.agents.get(conn.from), state.agents.get(conn.to)) {
                let start = a.pos + egui::vec2(50.0, 20.0);
                let end = b.pos + egui::vec2(50.0, 20.0);
                painter.line_segment([start, end], egui::Stroke::new(1.2, egui::Color32::from_gray(100)));

                if state.is_running {
                    let t = (time * 0.6 % 1.0) as f32;
                    let dot_pos = start + (end - start) * t;
                    painter.circle_filled(dot_pos, 4.0, egui::Color32::YELLOW);
                    ctx.request_repaint();
                }
            }
        }

        // --- 2. Render & Interaction ---
        for i in 0..state.agents.len() {
            let id = egui::Id::new("ag").with(i);
            let mut pos = state.agents[i].pos;
            let rect = egui::Rect::from_min_size(pos, egui::vec2(100.0, 40.0));
            
            // Visual feedback jika dipilih untuk di-link
            let stroke = if state.link_source == Some(i) {
                egui::Stroke::new(2.0, egui::Color32::GOLD)
            } else if state.selected_agent == Some(i) {
                egui::Stroke::new(2.0, egui::Color32::WHITE)
            } else {
                egui::Stroke::NONE
            };

            let resp = ui.interact(rect, id, egui::Sense::click_and_drag());
            
            if resp.dragged() {
                pos += resp.drag_delta();
                state.agents[i].pos = pos;
            }

            if resp.clicked() {
                if let Some(source_idx) = state.link_source {
                    if source_idx != i {
                        // Tambah garis manual
                        state.connections.push(Connection { 
                            from: source_idx, 
                            to: i, 
                            message: "Data Transfer".to_string() 
                        });
                    }
                    state.link_source = None; // Reset setelah connect
                } else {
                    state.selected_agent = Some(i);
                    state.show_kb = true;
                }
            }

            // Tahan tombol (Long Press simulasi lewat tombol lain atau double click)
            // Di sini kita pakai fitur: Klik Agent lalu klik tombol "LINK"
            
            painter.rect(rect, 8.0, state.agents[i].color, stroke);
            painter.text(rect.center(), egui::Align2::CENTER_CENTER, &state.agents[i].name, egui::FontId::proportional(14.0), egui::Color32::BLACK);
        }

        // Petunjuk Link
        if state.selected_agent.is_some() && state.link_source.is_none() {
            if ui.put(egui::Rect::from_min_size(egui::pos2(20.0, 120.0), egui::vec2(80.0, 30.0)), egui::Button::new("LINK")).clicked() {
                state.link_source = state.selected_agent;
            }
        }
    });
}
