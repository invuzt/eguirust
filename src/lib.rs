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
        Box::new(move |cc| {
            // Load Font agar teks tidak hilang (karena default_fonts dimatikan)
            let mut fonts = egui::FontDefinitions::default();
            fonts.font_data.insert(
                "custom_font".to_owned(),
                egui::FontData::from_static(include_bytes!("../assets/font.ttf")),
            );
            fonts.families.get_mut(&egui::FontFamily::Proportional)
                .unwrap()
                .insert(0, "custom_font".to_owned());
            fonts.families.get_mut(&egui::FontFamily::Monospace)
                .unwrap()
                .push("custom_font".to_owned());
            cc.egui_ctx.set_fonts(fonts);

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
