#![cfg(target_os = "android")]
use eframe::egui;
use std::sync::Arc;
use std::collections::HashMap;

struct Node {
    pos: egui::Pos2,
    vel: egui::Vec2,
    is_server: bool,
}

struct AppState {
    nodes: HashMap<String, Node>,
}

#[no_mangle]
fn android_main(app: android_activity::AndroidApp) {
    use winit::platform::android::EventLoopBuilderExtAndroid;
    android_logger::init_once(android_logger::Config::default().with_max_level(log::LevelFilter::Info));

    let state = Arc::new(std::sync::Mutex::new(AppState { nodes: HashMap::new() }));
    let mut options = eframe::NativeOptions::default();
    options.event_loop_builder = Some(Box::new(move |builder| {
        builder.with_android_app(app);
    }));

    let _ = eframe::run_native(
        "Odfiz Core", 
        options, 
        Box::new(move |_cc| {
            Box::new(OdfizApp { state }) as Box<dyn eframe::App>
        })
    );
}

struct OdfizApp {
    state: Arc<std::sync::Mutex<AppState>>,
}

impl eframe::App for OdfizApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("ODFIZ RT/RW NET");
            if ui.button("ADD NODE").clicked() {
                if let Ok(mut data) = self.state.lock() {
                    let id = data.nodes.len();
                    data.nodes.insert(format!("N-{}", id), Node { 
                        pos: egui::pos2(100.0, 100.0), vel: egui::Vec2::ZERO, is_server: id == 0 
                    });
                }
            }
            ui.label(format!("Active Nodes: {}", self.state.lock().unwrap().nodes.len()));
        });
    }
}
