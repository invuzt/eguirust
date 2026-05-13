use eframe::egui;
use crate::app_logic::{AppState, NodeType};

pub fn render_ui(ctx: &egui::Context, state: &mut AppState) {
    let screen_rect = ctx.input(|i| i.screen_rect());
    let top_margin = screen_rect.height() * 0.06;
    let bottom_margin = screen_rect.height() * 0.02;
    let side_margin = screen_rect.width() * 0.01;
    
    // Responsive button size based on screen width
    let btn_width = (screen_rect.width() / 6.5).max(50.0).min(80.0);
    let btn_height = 44.0;
    
    egui::TopBottomPanel::top("toolbar")
        .frame(egui::Frame::none().fill(egui::Color32::from_rgba_premultiplied(0,0,0,230)))
        .resizable(false)
        .show(ctx, |ui| {
            ui.add_space(top_margin);
            ui.horizontal(|ui| {
                ui.add_space(side_margin);
                
                let btn_style = |ui: &mut egui::Ui, text: &str, color: egui::Color32| {
                    let btn = egui::Button::new(egui::RichText::new(text).color(egui::Color32::WHITE).size(13.0))
                        .fill(color)
                        .rounding(egui::Rounding::same(25.0));
                    ui.add_sized(egui::vec2(btn_width, btn_height), btn).clicked()
                };
                
                if btn_style(ui, "➕ IN", egui::Color32::from_rgb(34, 197, 94)) {
                    state.add_node(NodeType::Input, egui::pos2(100.0, 200.0));
                }
                if btn_style(ui, "⚙ PRO", egui::Color32::from_rgb(59, 130, 246)) {
                    state.add_node(NodeType::Process, egui::pos2(300.0, 200.0));
                }
                if btn_style(ui, "📤 OUT", egui::Color32::from_rgb(239, 68, 68)) {
                    state.add_node(NodeType::Output, egui::pos2(500.0, 200.0));
                }
                if btn_style(ui, "🔧 FN", egui::Color32::from_rgb(168, 85, 247)) {
                    state.add_node(NodeType::Function, egui::pos2(700.0, 200.0));
                }
                
                ui.add_space(8.0);
                
                let run_color = if state.is_running { egui::Color32::from_rgb(220, 38, 38) } else { egui::Color32::from_rgb(34, 197, 94) };
                let run_text = if state.is_running { "⏹ STOP" } else { "▶ RUN" };
                if btn_style(ui, run_text, run_color) {
                    state.is_running = !state.is_running;
                    if state.is_running { state.run_execution(); }
                }
                
                if btn_style(ui, "🗑 DEL", egui::Color32::from_rgb(124, 58, 237)) {
                    state.delete_selected();
                }
                if btn_style(ui, "🔄 RST", egui::Color32::from_rgb(107, 114, 128)) {
                    state.view_offset = egui::vec2(0.0, 0.0);
                    state.zoom_factor = 1.0;
                }
                
                ui.add_space(side_margin);
            });
            ui.add_space(bottom_margin);
        });

    // Log panel - responsive width
    let log_width = (screen_rect.width() * 0.28).max(180.0).min(320.0);
    
    egui::SidePanel::right("log_panel")
        .resizable(true)
        .default_width(log_width)
        .frame(egui::Frame::none().fill(egui::Color32::from_rgba_premultiplied(15,15,20,245)))
        .show(ctx, |ui| {
            ui.add_space(top_margin + 4.0);
            ui.heading(egui::RichText::new("📋 LOG").color(egui::Color32::WHITE).size(14.0));
            ui.separator();
            ui.add_space(4.0);
            egui::ScrollArea::vertical().show(ui, |ui| {
                for log in state.execution_log.iter().rev().take(40) {
                    let color = if log.contains("null") { egui::Color32::from_rgb(150,150,150) } 
                                else if log.contains("error") { egui::Color32::from_rgb(255,100,100) }
                                else { egui::Color32::from_rgb(180,220,180) };
                    ui.label(egui::RichText::new(log).color(color).size(11.0));
                    ui.add_space(3.0);
                }
                if state.execution_log.is_empty() {
                    ui.colored_label(egui::Color32::from_gray(100), "▶ Tap RUN to execute");
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

            // Draw connections
            for conn in &state.connections {
                let from_node = state.nodes.iter().find(|n| n.id == conn.from_node);
                let to_node = state.nodes.iter().find(|n| n.id == conn.to_node);
                
                if let (Some(fn_node), Some(tn_node)) = (from_node, to_node) {
                    let start = to_screen(fn_node.pos + egui::vec2(150.0, 40.0));
                    let end = to_screen(tn_node.pos + egui::vec2(0.0, 40.0));
                    
                    painter.line_segment([start, end], egui::Stroke::new(3.0, egui::Color32::from_rgb(0, 180, 220)));
                    
                    // Arrow
                    let dir = (end - start).normalized();
                    let arrow_size = 12.0;
                    let perp = egui::vec2(-dir.y, dir.x);
                    
                    painter.line_segment([end, end - dir * arrow_size + perp * 5.0], 
                        egui::Stroke::new(2.0, egui::Color32::from_rgb(0, 180, 220)));
                    painter.line_segment([end, end - dir * arrow_size - perp * 5.0], 
                        egui::Stroke::new(2.0, egui::Color32::from_rgb(0, 180, 220)));
                    
                    // Animated flow
                    if state.is_running {
                        let t = (ctx.input(|i| i.time) * 1.8).fract() as f32;
                        let flow_pos = start + (end - start) * t;
                        painter.circle_filled(flow_pos, 6.0, egui::Color32::from_rgb(255, 180, 0));
                        ctx.request_repaint();
                    }
                }
            }
            
            // Temp connection
            if let Some((from_node_id, _, current_pos)) = &state.temp_connection {
                if let Some(from_node) = state.nodes.iter().find(|n| n.id == *from_node_id) {
                    let start = to_screen(from_node.pos + egui::vec2(150.0, 40.0));
                    painter.line_segment([start, *current_pos], egui::Stroke::new(3.0, egui::Color32::from_rgb(200, 200, 100)));
                }
            }

            // Draw nodes - responsive size
            let node_width = (screen_rect.width() / 8.0).min(150.0).max(110.0);
            let node_height = node_width * 0.55;
            
            for i in 0..state.nodes.len() {
                let id = egui::Id::new("node").with(i);
                let screen_pos = to_screen(state.nodes[i].pos);
                let size = egui::vec2(node_width, node_height) * zoom;
                let rect = egui::Rect::from_min_size(screen_pos, size);
                let resp = ui.interact(rect, id, egui::Sense::click_and_drag());
                
                if resp.dragged() { 
                    state.nodes[i].pos += resp.drag_delta() / zoom; 
                }
                if resp.clicked() { 
                    state.selected_node = Some(i); 
                }
                
                // Visual
                painter.rect_filled(rect.translate(egui::vec2(3.0, 3.0)), 12.0 * zoom, 
                    egui::Color32::from_rgba_premultiplied(0,0,0,80));
                painter.rect_filled(rect, 12.0 * zoom, state.nodes[i].color);
                painter.rect_stroke(rect, 12.0 * zoom, egui::Stroke::new(1.5, egui::Color32::WHITE));
                
                // Node name with ellipsis if too long
                let display_name = if state.nodes[i].name.len() > 12 {
                    format!("{}...", &state.nodes[i].name[..10])
                } else {
                    state.nodes[i].name.clone()
                };
                
                painter.text(
                    rect.center(), 
                    egui::Align2::CENTER_CENTER, 
                    display_name,
                    egui::FontId::proportional((13.0 * zoom).max(10.0)),
                    egui::Color32::WHITE
                );
                
                // Ports (touch-friendly size)
                let port_size = (node_width / 8.0).max(14.0).min(20.0) * zoom;
                
                // Output port
                let right_port = egui::Rect::from_min_size(
                    rect.max - egui::vec2(port_size * 0.5, node_height * 0.5) * zoom, 
                    egui::vec2(port_size, port_size)
                );
                let port_resp = ui.interact(right_port, id.with("out"), egui::Sense::click());
                painter.circle_filled(right_port.center(), port_size * 0.5, egui::Color32::from_rgb(255, 100, 100));
                painter.circle_stroke(right_port.center(), port_size * 0.5, egui::Stroke::new(1.5, egui::Color32::WHITE));
                
                if port_resp.clicked() {
                    state.temp_connection = Some((state.nodes[i].id.clone(), "output".to_string(), right_port.center()));
                }
                
                // Input port
                let left_port = egui::Rect::from_min_size(
                    rect.min + egui::vec2(-port_size * 0.5, node_height * 0.5) * zoom, 
                    egui::vec2(port_size, port_size)
                );
                let left_resp = ui.interact(left_port, id.with("in"), egui::Sense::click());
                painter.circle_filled(left_port.center(), port_size * 0.5, egui::Color32::from_rgb(100, 255, 100));
                painter.circle_stroke(left_port.center(), port_size * 0.5, egui::Stroke::new(1.5, egui::Color32::WHITE));
                
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
            
            // Instruction text
            if state.nodes.is_empty() {
                painter.text(
                    rect.center(),
                    egui::Align2::CENTER_CENTER,
                    "👆 Tap IN, PRO, OUT, or FN to add nodes\n🔗 Drag from red port → green port to connect\n▶ Press RUN to execute data flow",
                    egui::FontId::proportional(14.0),
                    egui::Color32::from_gray(120)
                );
            }
        });
}
