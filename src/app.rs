use egui::Context;

pub struct HomeApp {
    input_text: String,
    show_graph: bool,
}

impl Default for HomeApp {
    fn default() -> Self {
        Self {
            input_text: String::new(),
            show_graph: false,
        }
    }
}

impl HomeApp {
    pub fn update(&mut self, ctx: &Context) {
        let screen_rect = ctx.input(|i| i.screen_rect());
        let screen_height = screen_rect.height();
        
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(screen_height * 0.15);
                
                ui.heading("Vuzt Dashboard");
                ui.add_space(20.0);
                
                ui.label("Input Teks dengan Keyboard Native Android:");
                ui.add_space(10.0);
                
                // TextEdit dengan keyboard native Android
                let text_edit = egui::TextEdit::singleline(&mut self.input_text)
                    .hint_text("Ketik sesuatu di sini...")
                    .desired_width(300.0)
                    .font(egui::FontId::proportional(18.0));
                
                ui.add(text_edit);
                
                ui.add_space(20.0);
                
                if !self.input_text.is_empty() {
                    ui.label(format("Anda mengetik: {}", self.input_text));
                }
                
                ui.add_space(30.0);
                
                if ui.button("Lihat Grafik").clicked() {
                    self.show_graph = true;
                }
                
                ui.add_space(20.0);
                
                if ui.button("Clear").clicked() {
                    self.input_text.clear();
                }
            });
        });
    }
    
    pub fn show_graph(&self) -> bool {
        self.show_graph
    }
    
    pub fn set_show_graph(&mut self, value: bool) {
        self.show_graph = value;
    }
    
    pub fn get_input_text(&self) -> &str {
        &self.input_text
    }
}
