#![cfg(target_os = "android")]
use eframe::egui;

#[no_mangle]
fn android_main(app: android_activity::AndroidApp) {
    use winit::platform::android::EventLoopBuilderExtAndroid;
    android_logger::init_once(android_logger::Config::default().with_max_level(log::LevelFilter::Info));

    let mut options = eframe::NativeOptions::default();
    
    // INI KUNCINYA: Pakai event_loop_builder, jangan direct field
    options.event_loop_builder = Some(Box::new(move |builder| {
        builder.with_android_app(app);
    }));

    let _ = eframe::run_native(
        "Odfiz App",
        options,
        Box::new(|_cc| {
            // Kembalikan Box<dyn eframe::App> langsung
            Ok(Box::new(MyApp::default()))
        }),
    );
}

struct MyApp {
    nama: String,
    angka: i32,
    counter: i32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            nama: String::new(),
            angka: 0,
            counter: 0,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Odfiz Rust Native");
            ui.add_space(10.0);

            ui.label(format!("Counter: {}", self.counter));
            
            if ui.button("Tambah Counter").clicked() {
                self.counter += 1;
            }

            ui.separator();

            ui.horizontal(|ui| {
                ui.label("Nama: ");
                ui.text_edit_singleline(&mut self.nama);
            });

            ui.add(egui::DragValue::new(&mut self.angka).prefix("Angka: "));

            ui.add_space(20.0);
            if ui.button("Reset All").clicked() {
                self.counter = 0;
                self.nama.clear();
                self.angka = 0;
            }
        });
    }
}
