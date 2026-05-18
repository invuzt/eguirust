use eframe::egui;

pub fn apply_custom_style(ctx: &egui::Context) {
    let visuals = egui::Visuals::default();
    ctx.set_visuals(visuals);
    
    let mut style = (*ctx.style()).clone();
    style.spacing.item_spacing = egui::vec2(8.0, 8.0);
    ctx.set_style(style);
}
