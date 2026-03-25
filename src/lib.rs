use android_activity::{AndroidApp, MainEvent, PollEvent, InputStatus};
use std::time::Duration;

#[no_mangle]
fn android_main(app: AndroidApp) {
    let mut touch_count = 0;

    loop {
        app.poll_events(Some(Duration::from_millis(16)), |event| {
            match event {
                PollEvent::Main(MainEvent::Destroy) => {
                    return;
                }
                PollEvent::Wake => {
                    if let Ok(mut input_iter) = app.input_events_iter() {
                        while input_iter.next(|_input_event| {
                            touch_count += 1;
                            // Di sini logikanya: Setiap sentuhan terdeteksi, 
                            // variabel touch_count bertambah.
                            InputStatus::Handled
                        }) {}
                    }
                }
                _ => {}
            }
        });
    }
}
