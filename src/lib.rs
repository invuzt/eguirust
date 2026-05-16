#![cfg(target_os = "android")]
mod app;

use eframe::egui;
use android_activity::AndroidApp;

#[no_mangle]
fn android_main(app: AndroidApp) {
    android_logger::init_once(
        android_logger::Config::default().with_max_level(log::LevelFilter::Info)
    );

    let mut options = eframe::NativeOptions {
        renderer: eframe::Renderer::Glow,
        ..Default::default()
    };
    
    let app_clone = app.clone();
    options.event_loop_builder = Some(Box::new(move |builder| {
        use winit::platform::android::EventLoopBuilderExtAndroid;
        builder.with_android_app(app_clone);
    }));

    let _ = eframe::run_native(
        "Vuzt Keyboard Demo",
        options,
        Box::new(|_cc| Box::new(app::MyApp::default())),
    );
}
