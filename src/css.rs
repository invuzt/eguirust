use eframe::egui;

pub fn apply_custom_style(ctx: &egui::Context) {
    // Gunakan style default egui
    let visuals = egui::Visuals::default();
    ctx.set_visuals(visuals);
    
    // Set style minimal
    let mut style = (*ctx.style()).clone();
    style.spacing.item_spacing = egui::vec2(8.0, 8.0);
    style.spacing.window_margin = egui::Margin::same(8.0);
    ctx.set_style(style);
}
