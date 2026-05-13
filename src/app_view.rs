use eframe::egui;
use crate::app_logic::{AppState, NodeType};

pub fn render_ui(ctx: &egui::Context, state: &mut AppState) {
    let screen_rect = ctx.input(|i| i.screen_rect());
    let top_margin = screen_rect.height() * 0.08;
    
    egui::TopBottomPanel::top("toolbar")
        .frame(egui::Frame::none().fill(egui::Color32::from_rgba_premultiplied(0,0,0,220)))
        .resizable(false)
        .show(ctx, |ui| {
            ui.add_space(top_margin);
            ui.horizontal(|ui| {
                ui.add_space(12.0);
                
                let btn_style = |ui: &mut egui::Ui, text: &str, color: egui::Color32| {
                    let btn = egui::Button::new(egui::RichText::new(text).color(egui::Color32::WHITE).size(14.0))
                        .fill(color)
                        .rounding(egui::Rounding::same(30.0));
                    ui.add_sized(egui::vec2(65.0, 44.0), btn).clicked()
                };
                
                if btn_style(ui, "IN", egui::Color32::from_rgb(34, 197, 94)) {
                    state.add_node(NodeType::Input, egui::pos2(100.0, 250.0));
                }
                if btn_style(ui, "PRO", egui::Color32::from_rgb(59, 130, 246)) {
                    state.add_node(NodeType::Process, egui::pos2(300.0, 250.0));
                }
                if btn_style(ui, "OUT", egui::Color32::from_rgb(239, 68, 68)) {
                    state.add_node(NodeType::Output, egui::pos2(500.0, 250.0));
                }
                if btn_style(ui, "FN", egui::Color32::from_rgb(168, 85, 247)) {
                    state.add_node(NodeType::Function, egui::pos2(700.0, 250.0));
                }
                
                ui.add_space(12.0);
                
                let run_color = if state.is_running { egui::Color32::from_rgb(220, 38, 38) } else { egui::Color32::from_rgb(34, 197, 94) };
                if btn_style(ui, if state.is_running { "STOP" } else { "RUN" }, run_color) {
                    state.is_running = !state.is_running;
                    if state.is_running { state.run_execution(); }
                }
                
                if btn_style(ui, "DEL", egui::Color32::from_rgb(124, 58, 237)) {
                    state.delete_selected();
                }
                if btn_style(ui, "RESET", egui::Color32::from_rgb(107, 114, 128)) {
                    state.view_offset = egui::vec2(0.0, 0.0);
                    state.zoom_factor = 1.0;
                }
                ui.add_space(12.0);
            });
            ui.add_space(8.0);
        });

    egui::SidePanel::right("log_panel")
        .resizable(true)
        .default_width(280.0)
        .frame(egui::Frame::none().fill(egui::Color32::from_rgba_premultiplied(15,15,20,245)))
        .show(ctx, |ui| {
            ui.add_space(top_margin + 8.0);
            ui.heading(egui::RichText::new("📋 EVENT LOG").color(egui::Color32::WHITE).size(16.0));
            ui.separator();
            ui.add_space(8.0);
            egui::ScrollArea::vertical().show(ui, |ui| {
                for log in state.execution_log.iter().rev().take(50) {
                    let color = if log.contains("= null") { egui::Color32::from_rgb(150,150,150) } 
                                else { egui::Color32::from_rgb(200,220,255) };
                    ui.label(egui::RichText::new(log).color(color).size(12.0));
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

            // Draw existing connections
            for conn in &state.connections {
                let from_node = state.nodes.iter().find(|n| n.id == conn.from_node);
                let to_node = state.nodes.iter().find(|n| n.id == conn.to_node);
                if let (Some(fn_node), Some(tn_node)) = (from_node, to_node) {
                    let start = to_screen(fn_node.pos + egui::vec2(140.0, 35.0));
                    let end = to_screen(tn_node.pos + egui::vec2(0.0, 35.0));
                    
                    let cp1 = start + egui::vec2(60.0, 0.0);
                    let cp2 = end - egui::vec2(60.0, 0.0);
                    
                    let points: Vec<egui::Pos2> = (0..20).map(|i| {
                        let t = i as f32 / 19.0;
                        let x = (1.0-t).powi(2) * start.x + 2.0*(1.0-t)*t * cp1.x + t.powi(2) * cp2.x;
                        let y = (1.0-t).powi(2) * start.y + 2.0*(1.0-t)*t * cp1.y + t.powi(2) * cp2.y;
                        egui::pos2(x, y)
                    }).collect();
                    
                    for i in 0..points.len()-1 {
                        painter.line_segment([points[i], points[i+1]], egui::Stroke::new(3.0, egui::Color32::from_rgb(80, 150, 255)));
                    }
                    
                    if let (Some(last), Some(prev)) = (points.last(), points.get(points.len()-2)) {
                        let dir = (*last - *prev).normalized();
                        let arrow_size = 12.0;
                        painter.line_segment([*last, *last - dir * arrow_size + egui::vec2(-dir.y, dir.x) * 5.0], egui::Stroke::new(2.0, egui::Color32::from_rgb(80, 150, 255)));
                        painter.line_segment([*last, *last - dir * arrow_size - egui::vec2(-dir.y, dir.x) * 5.0], egui::Stroke::new(2.0, egui::Color32::from_rgb(80, 150, 255)));
                    }
                    
                    if state.is_running {
                        let t = (ctx.input(|i| i.time) * 1.5).fract() as f32;
                        let flow_x = (1.0-t).powi(2) * start.x + 2.0*(1.0-t)*t * cp1.x + t.powi(2) * cp2.x;
                        let flow_y = (1.0-t).powi(2) * start.y + 2.0*(1.0-t)*t * cp1.y + t.powi(2) * cp2.y;
                        let flow_point = egui::pos2(flow_x, flow_y);
                        painter.circle_filled(flow_point, 8.0, egui::Color32::from_rgb(255, 200, 0));
                        ctx.request_repaint();
                    }
                }
            }
            
            // Draw temp connection while dragging
            if let Some((from_node_id, _, current_pos)) = &state.temp_connection {
                if let Some(from_node) = state.nodes.iter().find(|n| n.id == *from_node_id) {
                    let start = to_screen(from_node.pos + egui::vec2(140.0, 35.0));
                    let end = *current_pos;
                    painter.line_segment([start, end], egui::Stroke::new(3.0, egui::Color32::from_rgb(150, 150, 200)));
                }
            }

            // Draw nodes
            for i in 0..state.nodes.len() {
                let id = egui::Id::new("node").with(i);
                let screen_pos = to_screen(state.nodes[i].pos);
                let size = egui::vec2(140.0, 70.0) * zoom;
                let rect = egui::Rect::from_min_size(screen_pos, size);
                let resp = ui.interact(rect, id, egui::Sense::click_and_drag());
                
                if resp.dragged() { 
                    state.nodes[i].pos += resp.drag_delta() / zoom; 
                }
                if resp.clicked() { 
                    state.selected_node = Some(i); 
                }
                
                let shadow_rect = rect.translate(egui::vec2(4.0, 4.0));
                painter.rect_filled(shadow_rect, 16.0 * zoom, egui::Color32::from_rgba_premultiplied(0,0,0,80));
                painter.rect_filled(rect, 16.0 * zoom, state.nodes[i].color);
                
                painter.text(
                    rect.center(), 
                    egui::Align2::CENTER_CENTER, 
                    &state.nodes[i].name,
                    egui::FontId::proportional(15.0 * zoom),
                    egui::Color32::WHITE
                );
                
                // Right port (output) - click to start connection
                let right_port = egui::Rect::from_min_size(rect.max - egui::vec2(6.0, 35.0) * zoom, egui::vec2(16.0, 16.0) * zoom);
                let port_resp = ui.interact(right_port, id.with("port"), egui::Sense::click());
                
                painter.circle_filled(right_port.center(), 8.0 * zoom, egui::Color32::from_rgb(255, 100, 100));
                
                if port_resp.clicked() {
                    state.temp_connection = Some((state.nodes[i].id.clone(), "output".to_string(), port_resp.interact_pointer_pos().unwrap_or(rect.max)));
                }
                
                // Left port (input) - release connection here
                let left_port = egui::Rect::from_min_size(rect.min + egui::vec2(-10.0, 35.0) * zoom, egui::vec2(16.0, 16.0) * zoom);
                let left_resp = ui.interact(left_port, id.with("inport"), egui::Sense::click());
                
                painter.circle_filled(left_port.center(), 8.0 * zoom, egui::Color32::from_rgb(100, 255, 100));
                
                if left_resp.clicked() && state.temp_connection.is_some() {
                    if let Some((from_id, from_port, _)) = state.temp_connection.take() {
                        state.add_connection(from_id, from_port, state.nodes[i].id.clone(), "input".to_string());
                    }
                }
            }
            
            // Cancel connection on background click
            if resp.clicked() && state.temp_connection.is_some() {
                state.temp_connection = None;
            }
        });
}
