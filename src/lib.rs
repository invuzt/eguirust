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
            // Mengatur zoom agar UI tidak terlalu kecil di layar HP DPI tinggi
            _cc.egui_ctx.set_pixels_per_point(1.2);
            Box::new(OdfizApp { state }) as Box<dyn eframe::App>
        })
    );
}

struct OdfizApp {
    state: Arc<std::sync::Mutex<AppState>>,
}

impl eframe::App for OdfizApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Menggunakan Dark Theme secara eksplisit
        ctx.set_visuals(egui::Visuals::dark());

        egui::CentralPanel::default().show(ctx, |ui| {
            // --- MARGIN UNTUK STATUS BAR ---
            ui.add_space(60.0); 

            // --- MEMBUAT LAYOUT DI TENAH ---
            ui.vertical_centered(|ui| {
                ui.heading("ODFIZ RT/RW NET");
                ui.add_space(20.0);

                ui.group(|ui| {
                    ui.set_max_width(300.0); // Membatasi lebar agar terlihat rapi
                    ui.label(format!("Nodes Terdeteksi: {}", self.state.lock().unwrap().nodes.len()));
                    
                    ui.add_space(10.0);
                    
                    if ui.add(egui::Button::new("➕ TAMBAH NODE").min_size(egui::vec2(200.0, 50.0))).clicked() {
                        if let Ok(mut data) = self.state.lock() {
                            let id = data.nodes.len();
                            data.nodes.insert(format!("N-{}", id), Node { 
                                pos: egui::pos2(0.0, 0.0), vel: egui::Vec2::ZERO, is_server: id == 0 
                            });
                        }
                    }

                    if ui.add(egui::Button::new("🔄 RESET ALL").min_size(egui::vec2(200.0, 40.0))).clicked() {
                         if let Ok(mut data) = self.state.lock() { data.nodes.clear(); }
                    }
                });

                ui.add_space(30.0);
                ui.separator();
                ui.add_space(20.0);
                
                ui.label("Status: Running on Rust Native");
            });
        });
        
        // Memaksa refresh UI jika ada perubahan physics kelak
        ctx.request_repaint();
    }
}
