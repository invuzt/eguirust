use eframe::egui;
use crate::app_logic::{AppState, NodeType};

pub fn render_ui(ctx: &egui::Context, state: &mut AppState) {
    egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.heading("VUZT DATA FLOW");
            if ui.button("➕ INPUT").clicked() {
                state.add_node(NodeType::Input, egui::pos2(100.0, 200.0));
            }
            if ui.button("⚙ PROCESS").clicked() {
                state.add_node(NodeType::Process, egui::pos2(300.0, 200.0));
            }
            if ui.button("📤 OUTPUT").clicked() {
                state.add_node(NodeType::Output, egui::pos2(500.0, 200.0));
            }
            if ui.button("🔧 FUNCTION").clicked() {
                state.add_node(NodeType::Function, egui::pos2(700.0, 200.0));
            }
            if ui.button(if state.is_running { "⏹ STOP" } else { "▶ RUN" }).clicked() {
                state.is_running = !state.is_running;
                if state.is_running { state.run_execution(); }
            }
            if ui.button("🗑 DELETE").clicked() { state.delete_selected(); }
            if ui.button("🔄 RESET VIEW").clicked() {
                state.view_offset = egui::vec2(0.0, 0.0);
                state.zoom_factor = 1.0;
            }
        });
    });

    egui::SidePanel::right("log_panel").resizable(true).default_width(250.0).show(ctx, |ui| {
        ui.heading("Execution Log");
        ui.separator();
        for log in state.execution_log.iter().rev().take(20) {
            ui.label(log);
        }
    });

    egui::CentralPanel::default().show(ctx, |ui| {
        let rect = ui.max_rect();
        let resp = ui.interact(rect, ui.id(), egui::Sense::drag());
        if resp.dragged() && !ctx.input(|i| i.pointer.any_down()) {
            state.view_offset += resp.drag_delta();
        }
        
        let zoom_delta = ctx.input(|i| i.zoom_delta());
        if zoom_delta != 1.0 {
            state.zoom_factor *= zoom_delta;
            state.zoom_factor = state.zoom_factor.clamp(0.2, 3.0);
        }

        let painter = ui.painter();
        let zoom = state.zoom_factor;
        let offset = state.view_offset;
        let to_screen = |p: egui::Pos2| (p.to_vec2() * zoom).to_pos2() + offset;

        for conn in &state.connections {
            let from_node = state.nodes.iter().find(|n| n.id == conn.from_node);
            let to_node = state.nodes.iter().find(|n| n.id == conn.to_node);
            if let (Some(fn_node), Some(tn_node)) = (from_node, to_node) {
                let start = to_screen(fn_node.pos + egui::vec2(120.0, 30.0));
                let end = to_screen(tn_node.pos + egui::vec2(0.0, 30.0));
                painter.line_segment([start, end], egui::Stroke::new(2.0, egui::Color32::from_rgb(100, 100, 255)));
            }
        }

        for i in 0..state.nodes.len() {
            let id = egui::Id::new("node").with(i);
            let screen_pos = to_screen(state.nodes[i].pos);
            let size = egui::vec2(120.0, 60.0) * zoom;
            let rect = egui::Rect::from_min_size(screen_pos, size);
            let resp = ui.interact(rect, id, egui::Sense::click_and_drag());
            
            if resp.dragged() { state.nodes[i].pos += resp.drag_delta() / zoom; }
            if resp.clicked() { state.selected_node = Some(i); state.show_kb = true; }
            
            painter.rect_filled(rect, 8.0 * zoom, state.nodes[i].color);
            painter.text(rect.center(), egui::Align2::CENTER_CENTER, &state.nodes[i].name, egui::FontId::proportional(14.0 * zoom), egui::Color32::WHITE);
        }
    });
}
