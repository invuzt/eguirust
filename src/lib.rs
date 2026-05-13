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
        vsync: true, // Aktifkan VSync untuk menghemat baterai
        ..Default::default()
    };
    
    let app_clone = app.clone();
    options.event_loop_builder = Some(Box::new(move |builder| {
        use winit::platform::android::EventLoopBuilderExtAndroid;
        builder.with_android_app(app_clone);
    }));

    let _ = eframe::run_native(
        "Vuzt",
        options,
        Box::new(|cc| {
            let mut fonts = egui::FontDefinitions::default();
            fonts.font_data.insert(
                "custom_font".to_owned(),
                egui::FontData::from_static(include_bytes!("../assets/font.ttf")),
            );
            fonts.families.get_mut(&egui::FontFamily::Proportional)
                .unwrap()
                .insert(0, "custom_font".to_owned());
            cc.egui_ctx.set_fonts(fonts);
            
            // Set repaint interval untuk idle (render 2x per detik)
            cc.egui_ctx.request_repaint_after(std::time::Duration::from_millis(500));
            
            Box::new(app::MyApp::default())
        }),
    );
}
