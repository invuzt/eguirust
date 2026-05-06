use eframe::egui;
use crate::app_logic::AppState;

pub fn render_ui(ctx: &egui::Context, state: &mut AppState) {
    egui::TopBottomPanel::top("tp")
        .frame(egui::Frame::none().fill(egui::Color32::from_rgb(10, 10, 10)))
        .show(ctx, |ui| {
        ui.add_space(55.0);
        ui.horizontal(|ui| {
            ui.heading("VUZT CANVAS");
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("ADD").clicked() { state.add_agent(); }
                if ui.button(if state.is_running { "STOP" } else { "RUN" }).clicked() { state.is_running = !state.is_running;}
                
                // Tombol Reset View
                if ui.button("RESET").clicked() {
                    state.view_offset = egui::vec2(0.0, 0.0);
                    state.zoom_factor = 1.0;
                }

                let del_btn = egui::Button::new(egui::RichText::new("DEL").color(egui::Color32::WHITE))
                    .fill(egui::Color32::from_rgb(150, 0, 0));
                if ui.add(del_btn).clicked() { state.delete_selected(); }
            });
        });
    });

    egui::CentralPanel::default()
        .frame(egui::Frame::none().fill(egui::Color32::from_rgb(20, 20, 20)))
        .show(ctx, |ui| {
        
        let rect = ui.max_rect();
        let resp = ui.interact(rect, ui.id(), egui::Sense::drag());
        
        // Perbaikan: Menggunakan any_down() atau cek any_click() sesuai saran compiler
        // Kita gunakan any_down() untuk mendeteksi tekanan jari pada background
        if resp.dragged() && !ctx.input(|i|i.pointer.any_down()){
             state.view_offset += resp.drag_delta();
        }

        let zoom_delta = ctx.input(|i| i.zoom_delta());
        if zoom_delta != 1.0 {
            state.zoom_factor *= zoom_delta;
            state.zoom_factor = state.zoom_factor.clamp(0.2, 3.0);
        }

        let painter = ui.painter();
        let time = ctx.input(|i| i.time);
        let zoom = state.zoom_factor;
        let offset = state.view_offset;

        let to_screen = |p: egui::Pos2| (p.to_vec2() * zoom).to_pos2() + offset;

        // --- 1. Connections ---
        for conn in &state.connections {
            if let (Some(a), Some(b)) = (state.agents.get(conn.from), state.agents.get(conn.to)) {
                let start = to_screen(a.pos + egui::vec2(100.0, 20.0));
                let end = to_screen(b.pos + egui::vec2(0.0, 20.0));
                painter.line_segment([start, end], egui::Stroke::new(1.0 * zoom, egui::Color32::from_gray(80)));

                if state.is_running {
                    let t = (time * 0.5 % 1.0) as f32;
                    painter.circle_filled(start + (end - start) * t, 4.0 * zoom, egui::Color32::YELLOW);
                    ctx.request_repaint();
                }
            }
        }

        // --- 2. Agents ---
        let mut spawn_target = None;
        for i in 0..state.agents.len() {
            let id = egui::Id::new("ag").with(i);
            let screen_pos = to_screen(state.agents[i].pos);
            let agent_size = egui::vec2(100.0, 40.0) * zoom;
            let agent_rect = egui::Rect::from_min_size(screen_pos, agent_size);
            
            let resp = ui.interact(agent_rect, id, egui::Sense::click_and_drag());
            if resp.dragged() { 
                state.agents[i].pos += resp.drag_delta() / zoom; 
            }
            if resp.clicked() { 
                state.selected_agent = Some(i); 
                state.show_kb = true; 
            }

            painter.rect_filled(agent_rect, 6.0 * zoom, state.agents[i].color);
            painter.text(agent_rect.center(), egui::Align2::CENTER_CENTER, &state.agents[i].name, egui::FontId::proportional(14.0 * zoom), egui::Color32::BLACK);

            // Tombol (+) Child
            let plus_rect = egui::Rect::from_min_size(screen_pos + egui::vec2(105.0 * zoom, 5.0 * zoom), egui::vec2(30.0, 30.0) * zoom);
            let plus_resp = ui.interact(plus_rect, id.with("p"), egui::Sense::click());
            painter.rect_filled(plus_rect, egui::Rounding::same(4.0 * zoom), egui::Color32::from_rgb(50, 50, 50));
            painter.text(plus_rect.center(), egui::Align2::CENTER_CENTER, "+", egui::FontId::proportional(18.0 * zoom), egui::Color32::GREEN);

            if plus_resp.clicked() { spawn_target = Some(i); }
        }

        if let Some(idx) = spawn_target { state.spawn_child(idx); }
    });
}
