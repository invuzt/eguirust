#![cfg(target_os = "android")]

mod app;
mod graph;

use android_activity::AndroidApp;
use egui_and_android::Application;

#[no_mangle]
fn android_main(app: AndroidApp) {
    android_logger::init_once(
        android_logger::Config::default()
            .with_min_level(log::LevelFilter::Info)
            .with_tag("vuzt"),
    );

    let app_state = app::HomeApp::default();
    let mut application = Application::new(app, app_state);
    
    application.run();
}
