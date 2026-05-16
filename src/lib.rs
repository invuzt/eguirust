#![cfg(target_os = "android")]

slint::include_modules!();

mod egui_canvas;

use android_activity::AndroidApp;
use std::rc::Rc;

#[no_mangle]
fn android_main(app: AndroidApp) {
    android_logger::init_once(
        android_logger::Config::default().with_max_level(log::LevelFilter::Info)
    );

    let main_window = MainWindow::new().unwrap();
    
    // Clone untuk dipakai di callback
    let main_window_weak = main_window.as_weak();
    
    // Setup callback untuk Egui
    let egui_handler = egui_canvas::EguiHandler::new();
    let egui_handler_rc = Rc::new(egui_handler);
    
    // Handle input dari Slint ke Egui
    let main_window_weak2 = main_window.as_weak();
    let egui_handler_clone = egui_handler_rc.clone();
    
    // Simpan handler untuk digunakan nanti
    // (Dalam implementasi nyata, perlu thread atau channel)
    
    main_window.run().unwrap();
}
