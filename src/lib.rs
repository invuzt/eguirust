#![cfg(target_os = "android")]
mod app_logic;
mod app_view;

use eframe::egui;
use std::sync::{Arc, Mutex};
use android_activity::AndroidApp;

struct OdfizApp {
    state: Arc<Mutex<app_logic::AppState>>,
    android_app: AndroidApp,
}

#[no_mangle]
fn android_main(app: AndroidApp) {
    android_logger::init_once(android_logger::Config::default().with_max_level(log::LevelFilter::Info));

    let mut options = eframe::NativeOptions::default();
    let app_clone = app.clone();
    
    options.event_loop_builder = Some(Box::new(move |builder| {
        use winit::platform::android::EventLoopBuilderExtAndroid;
        builder.with_android_app(app_clone);
    }));

    let state = Arc::new(Mutex::new(app_logic::AppState::new()));
    let state_inner = state.clone();

    let _ = eframe::run_native(
        "Vuzt",
        options,
        Box::new(move |cc| {
            // Load Font (Wajib karena default_fonts = false)
            let mut fonts = egui::FontDefinitions::default();
            fonts.font_data.insert(
                "custom_font".to_owned(),
                egui::FontData::from_static(include_bytes!("../assets/font.ttf")),
            );
            fonts.families.get_mut(&egui::FontFamily::Proportional).unwrap().insert(0, "custom_font".to_owned());
            fonts.families.get_mut(&egui::FontFamily::Monospace).unwrap().push("custom_font".to_owned());
            cc.egui_ctx.set_fonts(fonts);

            Box::new(OdfizApp { 
                state: state_inner,
                android_app: app,
            }) as Box<dyn eframe::App>
        }),
    );
}

impl eframe::App for OdfizApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if let Ok(mut state) = self.state.lock() {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(50.0);
                    ui.heading("VUZT KEYBOARD TEST");
                    
                    // Tombol Pancingan
                    if ui.add_sized([200.0, 50.0], egui::Button::new("⌨ OPEN KEYBOARD")).clicked() {
                        // Meminta android-activity untuk menampilkan soft input
                        self.android_app.show_soft_input(true);
                    }

                    ui.add_space(20.0);
                    ui.label("Coba ketik di bawah:");
                    // TextEdit biasanya otomatis memicu keyboard jika focused
                    ui.text_edit_singleline(&mut state.app_name);
                });
            });
        }
    }
}
