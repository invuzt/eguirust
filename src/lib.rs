use android_activity::{AndroidApp, MainEvent, PollEvent};
use log::info;
use std::time::Duration;

#[no_mangle]
fn android_main(app: AndroidApp) {
    android_logger::init_once(
        android_logger::Config::default().with_max_level(log::LevelFilter::Info),
    );

    info!("Aplikasi Odfiz Native Dimulai! Coba sentuh layarnya.");

    loop {
        // Kita ambil semua event yang masuk
        app.poll_events(Some(Duration::from_millis(16)), |event| {
            match event {
                PollEvent::Main(MainEvent::Destroy) => {
                    info!("Aplikasi ditutup.");
                }
                // Cara baru akses input event di versi 0.6
                PollEvent::Wake => {
                    // Cek input secara manual dari queue
                    let mut input_iter = app.input_events_iter().unwrap();
                    while let Some(input_event) = input_iter.next() {
                        info!("Ada input masuk ke HP: {:?}", input_event);
                    }
                }
                _ => {}
            }
        });
    }
}
