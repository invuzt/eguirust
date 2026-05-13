use eframe::egui;

pub fn apply_custom_style(ctx: &egui::Context) {
    let mut visuals = egui::Visuals::dark();
    
    visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(30, 30, 35);
    visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(45, 45, 50);
    visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(66, 66, 72);
    visuals.widgets.active.bg_fill = egui::Color32::from_rgb(88, 88, 95);
    
    visuals.widgets.inactive.rounding = egui::Rounding::same(24.0);
    visuals.widgets.hovered.rounding = egui::Rounding::same(24.0);
    visuals.widgets.active.rounding = egui::Rounding::same(24.0);
    
    visuals.button_frame = true;
    
    ctx.set_visuals(visuals);
    
    let mut style = (*ctx.style()).clone();
    style.spacing.button_padding = egui::vec2(16.0, 12.0);
    style.spacing.item_spacing = egui::vec2(12.0, 12.0);
    ctx.set_style(style);
}
