use eframe::egui;

pub fn render_keyboard(ui: &mut egui::Ui, target_text: &mut String, show_kb: &mut bool) {
    ui.group(|ui| {
        ui.style_mut().spacing.item_spacing = egui::vec2(6.0, 8.0);
        
        let rows = [
            vec!["1","2","3","4","5","6","7","8","9","0"],
            vec!["Q","W","E","R","T","Y","U","I","O","P"],
            vec!["A","S","D","F","G","H","J","K","L","@"],
            vec!["Z","X","C","V","B","N","M",".","_","-"],
        ];

        for row in rows {
            ui.horizontal(|ui| {
                for key in row {
                    if ui.add_sized([32.0, 42.0], egui::Button::new(key)).clicked() {
                        target_text.push_str(&key.to_lowercase());
                    }
                }
            });
        }

        ui.horizontal(|ui| {
            if ui.add_sized([100.0, 42.0], egui::Button::new("SPASI")).clicked() {
                target_text.push(' ');
            }
            if ui.add_sized([100.0, 42.0], egui::Button::new("HAPUS")).clicked() {
                target_text.pop();
            }
            if ui.add_sized([100.0, 42.0], egui::Button::new("TUTUP")).clicked() {
                *show_kb = false;
            }
        });
    });
}
