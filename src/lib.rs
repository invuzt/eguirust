use android_activity::{AndroidApp, MainEvent, PollEvent, InputStatus};
use log::info;
use std::time::Duration;

#[no_mangle]
fn android_main(app: AndroidApp) {
    android_logger::init_once(
        android_logger::Config::default().with_max_level(log::LevelFilter::Info),
    );

    info!("Odfiz Native: Siap menerima input!");

    loop {
        app.poll_events(Some(Duration::from_millis(16)), |event| {
            match event {
                PollEvent::Main(MainEvent::Destroy) => {
                    info!("Aplikasi ditutup.");
                }
                PollEvent::Wake => {
                    if let Ok(mut input_iter) = app.input_events_iter() {
                        while input_iter.next(|input_event| {
                            info!("Input terdeteksi: {:?}", input_event);
                            // Kunci perbaikan: Kita harus mengembalikan status ini
                            InputStatus::Handled
                        }) {}
                    }
                }
                _ => {}
            }
        });
    }
}
