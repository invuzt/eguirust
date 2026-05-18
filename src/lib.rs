#![cfg(target_os = "android")]
mod app_logic;
mod keyboard;
mod css;
mod app_view;

use eframe::egui;
use std::sync::{Arc, Mutex};
use android_activity::AndroidApp;

struct VuztApp {
    state: Arc<Mutex<app_logic::AppState>>,
}

#[no_mangle]
fn android_main(app: AndroidApp) {
    android_logger::init_once(
        android_logger::Config::default().with_max_level(log::LevelFilter::Info)
    );

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
            crate::css::apply_custom_style(&cc.egui_ctx);
            Box::new(VuztApp { state: state_inner }) as Box<dyn eframe::App>
        }),
    );
}

impl eframe::App for VuztApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut state = self.state.lock().unwrap();
        crate::app_view::render_ui(ctx, &mut state);

        if state.show_kb {
            egui::TopBottomPanel::bottom("keyboard")
                .resizable(false)
                .default_height(250.0)
                .show(ctx, |ui| {
                    crate::keyboard::render_keyboard(ui, &mut state);
                });
        }
    }
}
