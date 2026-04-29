use eframe::{egui, NativeOptions};

#[no_mangle]
fn android_main(app: eframe::winit::platform::android::activity::AndroidApp) {
    android_logger::init_once(
        android_logger::Config::default().with_max_level(log::LevelFilter::Info),
    );

    let options = NativeOptions {
        android_app: Some(app),
        ..Default::default()
    };

    eframe::run_native(
        "Odfiz App",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    ).unwrap();
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

            if ui.button("Reset All").clicked() {
                self.counter = 0;
                self.nama.clear();
                self.angka = 0;
            }
        });
    }
}
