use eframe::egui;

pub fn apply_custom_style(ctx: &egui::Context) {
    let mut visuals = egui::Visuals::light();
    
    visuals.override_text_color = Some(egui::Color32::BLACK);
    visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(250, 250, 250);
    visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(230, 230, 230);
    visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(210, 210, 255);
    visuals.widgets.active.bg_fill = egui::Color32::from_rgb(180, 180, 255);
    visuals.selection.bg_fill = egui::Color32::from_rgb(150, 150, 255);
    
    ctx.set_visuals(visuals);
    
    let mut style = (*ctx.style()).clone();
    
    style.visuals.widgets.inactive.rounding = egui::Rounding::same(8.0);
    style.visuals.widgets.hovered.rounding = egui::Rounding::same(8.0);
    style.visuals.widgets.active.rounding = egui::Rounding::same(8.0);
    
    use egui::{FontId, FontFamily::Proportional};
    style.text_styles = [
        (egui::TextStyle::Heading, FontId::new(24.0, Proportional)),
        (egui::TextStyle::Name("Heading2".into()), FontId::new(20.0, Proportional)),
        (egui::TextStyle::Body, FontId::new(16.0, Proportional)),
        (egui::TextStyle::Button, FontId::new(14.0, Proportional)),
        (egui::TextStyle::Small, FontId::new(12.0, Proportional)),
    ].into();
    
    style.spacing.item_spacing = egui::vec2(10.0, 12.0);
    style.spacing.button_padding = egui::vec2(12.0, 8.0);
    
    ctx.set_style(style);
}
