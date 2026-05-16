#![cfg(target_os = "android")]

slint::include_modules!();

use android_activity::AndroidApp;

#[no_mangle]
fn android_main(app: AndroidApp) {
    android_logger::init_once(
        android_logger::Config::default().with_max_level(log::LevelFilter::Info)
    );

    slint::platform::set_platform(Box::new(slint::platform::android::AndroidPlatform::new(app))).unwrap();

    let main_window = MainWindow::new().unwrap();
    main_window.run().unwrap();
}
