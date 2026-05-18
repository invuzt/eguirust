use eframe::egui;

pub fn apply_custom_style(ctx: &egui::Context) {
    // Gunakan style default egui saja, tanpa modifikasi warna
    let visuals = egui::Visuals::default();
    ctx.set_visuals(visuals);
    
    // Hanya set style minimal tanpa mengubah warna
    let mut style = (*ctx.style()).clone();
    style.spacing.item_spacing = egui::vec2(5.0, 5.0);
    ctx.set_style(style);
}
