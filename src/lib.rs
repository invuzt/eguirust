#![cfg(target_os = "android")]
mod app_logic;
mod app_view;

use eframe::egui;
use std::sync::Arc;
use std::sync::Mutex;

struct OdfizApp {
    state: Arc<Mutex<app_logic::AppState>>,
}

#[no_mangle]
fn android_main(app: android_activity::AndroidApp) {
    use winit::platform::android::EventLoopBuilderExtAndroid;
    android_logger::init_once(android_logger::Config::default().with_max_level(log::LevelFilter::Info));

    let mut options = eframe::NativeOptions::default();
    options.event_loop_builder = Some(Box::new(move |builder| {
        builder.with_android_app(app);
    }));

    let state = Arc::new(Mutex::new(app_logic::AppState::new()));

    let _ = eframe::run_native(
        "Odfiz App",
        options,
        Box::new(move |_cc| {
            // Catat: Jika default_fonts mati, teks akan hilang 
            // kecuali kamu load font manual di sini menggunakan _cc.egui_ctx.set_fonts()
            Box::new(OdfizApp { state }) as Box<dyn eframe::App>
        }),
    );
}

impl eframe::App for OdfizApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if let Ok(mut state) = self.state.lock() {
            app_view::render_ui(ctx, &mut state);
        }
    }
}
