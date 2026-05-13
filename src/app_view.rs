use eframe::egui;
use crate::app_logic::{AppState, NodeType};

pub fn render_ui(ctx: &egui::Context, state: &mut AppState) {
    egui::TopBottomPanel::top("toolbar")
        .frame(egui::Frame::none().fill(egui::Color32::from_rgba_premultiplied(0,0,0,230)))
        .resizable(false)
        .show(ctx, |ui| {
            ui.add_space(45.0);
            ui.horizontal(|ui| {
                ui.add_space(12.0);
                
                let btn_style = |ui: &mut egui::Ui, text: &str, color: egui::Color32| {
                    let btn = egui::Button::new(egui::RichText::new(text).color(egui::Color32::WHITE))
                        .fill(color)
                        .rounding(egui::Rounding::same(20.0));
                    ui.add_sized(egui::vec2(70.0, 40.0), btn).clicked()
                };
                
                if btn_style(ui, "➕ IN", egui::Color32::from_rgb(34, 197, 94)) {
                    state.add_node(NodeType::Input, egui::pos2(100.0, 250.0));
                }
                if btn_style(ui, "⚙ PRO", egui::Color32::from_rgb(59, 130, 246)) {
                    state.add_node(NodeType::Process, egui::pos2(300.0, 250.0));
                }
                if btn_style(ui, "📤 OUT", egui::Color32::from_rgb(239, 68, 68)) {
                    state.add_node(NodeType::Output, egui::pos2(500.0, 250.0));
                }
                if btn_style(ui, "🔧 FN", egui::Color32::from_rgb(168, 85, 247)) {
                    state.add_node(NodeType::Function, egui::pos2(700.0, 250.0));
                }
                
                ui.add_space(20.0);
                
                let run_color = if state.is_running { egui::Color32::from_rgb(220, 38, 38) } else { egui::Color32::from_rgb(34, 197, 94) };
                if btn_style(ui, if state.is_running { "⏹ STOP" } else { "▶ RUN" }, run_color) {
                    state.is_running = !state.is_running;
                    if state.is_running { state.run_execution(); }
                }
                
                if btn_style(ui, "🗑 DEL", egui::Color32::from_rgb(124, 58, 237)) {
                    state.delete_selected();
                }
                if btn_style(ui, "🔄 RESET", egui::Color32::from_rgb(107, 114, 128)) {
                    state.view_offset = egui::vec2(0.0, 0.0);
                    state.zoom_factor = 1.0;
                }
                ui.add_space(12.0);
            });
            ui.add_space(8.0);
        });

    egui::SidePanel::right("log_panel")
        .resizable(true)
        .default_width(260.0)
        .frame(egui::Frame::none().fill(egui::Color32::from_rgba_premultiplied(20,20,25,240)))
        .show(ctx, |ui| {
            ui.add_space(55.0);
            ui.heading(egui::RichText::new("📋 EVENT LOG").color(egui::Color32::WHITE));
            ui.separator();
            ui.add_space(8.0);
            egui::ScrollArea::vertical().show(ui, |ui| {
                for log in state.execution_log.iter().rev().take(30) {
                    ui.label(egui::RichText::new(log).color(egui::Color32::from_rgb(200,200,210)));
                    ui.add_space(4.0);
                }
            });
        });

    egui::CentralPanel::default()
        .frame(egui::Frame::none().fill(egui::Color32::from_rgb(18, 18, 22)))
        .show(ctx, |ui| {
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
                    painter.line_segment([start, end], egui::Stroke::new(3.0, egui::Color32::from_rgb(100, 100, 255)));
                }
            }

            for i in 0..state.nodes.len() {
                let id = egui::Id::new("node").with(i);
                let screen_pos = to_screen(state.nodes[i].pos);
                let size = egui::vec2(130.0, 70.0) * zoom;
                let rect = egui::Rect::from_min_size(screen_pos, size);
                let resp = ui.interact(rect, id, egui::Sense::click_and_drag());
                
                if resp.dragged() { state.nodes[i].pos += resp.drag_delta() / zoom; }
                if resp.clicked() { state.selected_node = Some(i); state.show_kb = true; }
                
                painter.rect_filled(rect, 16.0 * zoom, state.nodes[i].color);
                painter.text(rect.center(), egui::Align2::CENTER_CENTER, &state.nodes[i].name, egui::FontId::proportional(16.0 * zoom), egui::Color32::WHITE);
            }
        });
}
