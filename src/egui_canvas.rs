use eframe::egui;

pub struct EguiHandler {
    text: String,
    counter: i32,
}

impl EguiHandler {
    pub fn new() -> Self {
        Self {
            text: String::new(),
            counter: 0,
        }
    }
    
    pub fn render(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Egui Canvas");
                ui.separator();
                ui.add_space(20.0);
                
                ui.label("Ini adalah Egui yang di-render di dalam Slint!");
                ui.add_space(20.0);
                
                ui.horizontal(|ui| {
                    ui.label("Counter: ");
                    ui.add(egui::DragValue::new(&mut self.counter).speed(1.0));
                });
                
                ui.add_space(20.0);
                
                let text_edit = egui::TextEdit::singleline(&mut self.text)
                    .hint_text("Ketik di sini...");
                ui.add(text_edit);
                
                ui.add_space(20.0);
                
                if ui.button("Click Me!").clicked() {
                    self.counter += 1;
                }
                
                ui.add_space(20.0);
                
                if !self.text.is_empty() {
                    ui.label(format!("Anda mengetik: {}", self.text));
                }
                
                ui.add_space(20.0);
                ui.label("Fitur: Zoom, Pan, Real-time update");
            });
        });
        
        // Request repaint untuk animasi
        ctx.request_repaint();
    }
}
