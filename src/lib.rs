#![cfg(target_os = "android")]

slint::include_modules!();

use android_activity::AndroidApp;

#[no_mangle]
fn android_main(app: AndroidApp) {
    android_logger::init_once(
        android_logger::Config::default().with_max_level(log::LevelFilter::Info)
    );

    let main_window = MainWindow::new().unwrap();
    main_window.run().unwrap();
}
