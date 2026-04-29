#![cfg(target_os = "android")]
use eframe::egui;
use std::sync::Arc;
use std::collections::HashMap;

struct Node {
    pos: egui::Pos2,
    vel: egui::Vec2,
    is_server: bool,
}

#[derive(Clone)]
struct Link {
    from: String,
    to: String,
    is_active: bool,
}

struct AppState {
    nodes: HashMap<String, Node>,
    links: Vec<Link>,
}

#[no_mangle]
fn android_main(app: android_activity::AndroidApp) {
    use winit::platform::android::EventLoopBuilderExtAndroid;
    android_logger::init_once(android_logger::Config::default().with_max_level(log::LevelFilter::Info));

    let state = Arc::new(std::sync::Mutex::new(AppState {
        nodes: HashMap::new(),
        links: Vec::new(),
    }));

    let mut options = eframe::NativeOptions::default();
    options.event_loop_builder = Some(Box::new(move |builder| {
        builder.with_android_app(app);
    }));

    let _ = eframe::run_native(
        "Odfiz Core", 
        options, 
        Box::new(move |_cc| {
            // PERBAIKAN: Langsung kembalikan Box tanpa Result (Ok)
            Box::new(OdfizApp { state, drag_node: None }) as Box<dyn eframe::App>
        })
    );
}

struct OdfizApp {
    state: Arc<std::sync::Mutex<AppState>>,
    drag_node: Option<String>,
}

impl eframe::App for OdfizApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_visuals(egui::Visuals::dark());
        let dt = 0.016;
        let time = ctx.input(|i| i.time);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(20.0);
            ui.heading("ODFIZ RT/RW NET SIMULATOR");

            ui.horizontal(|ui| {
                if ui.button("🖥 SET SERVER").clicked() { self.reset_and_add_server(); }
                if ui.button("📱 ADD CLIENT").clicked() { self.add_client(); }
            });

            let (rect, response) = ui.allocate_at_least(ui.available_size(), egui::Sense::drag());
            let painter = ui.painter_at(rect);
            let center = rect.center();

            if let Ok(mut data) = self.state.try_lock() {
                let names: Vec<String> = data.nodes.keys().cloned().collect();

                // Logic Physics & Interaction (Simplified for brevity)
                for name in &names {
                    let node = data.nodes.get_mut(name).unwrap();
                    if self.drag_node.as_ref() != Some(name) {
                        node.vel += (center - node.pos) * 0.2 * dt;
                        node.vel *= 0.95;
                        node.pos += node.vel;
                    }
                    
                    let color = if node.is_server { egui::Color32::GOLD } else { egui::Color32::LIGHT_BLUE };
                    painter.circle_filled(node.pos, 15.0, color);
                    painter.text(node.pos, egui::Align2::CENTER_CENTER, name, egui::FontId::proportional(12.0), egui::Color32::WHITE);
                }
            }
        });
        ctx.request_repaint();
    }
}

impl OdfizApp {
    fn reset_and_add_server(&self) {
        if let Ok(mut data) = self.state.try_lock() {
            data.nodes.clear();
            data.nodes.insert("SRV-01".into(), Node { pos: egui::pos2(200.0, 300.0), vel: egui::Vec2::ZERO, is_server: true });
        }
    }
    fn add_client(&self) {
        if let Ok(mut data) = self.state.try_lock() {
            let id = data.nodes.len();
            data.nodes.insert(format!("USR-{}", id), Node { 
                pos: egui::pos2(rand::random::<f32>() * 200.0, rand::random::<f32>() * 400.0), 
                vel: egui::Vec2::ZERO, is_server: false 
            });
        }
    }
}
