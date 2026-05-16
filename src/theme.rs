use eframe::egui;

pub fn apply_custom_style(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();
    
    // Font styling
    style.text_styles = [
        (egui::TextStyle::Heading, egui::FontId::proportional(24.0)),
        (egui::TextStyle::Name("Heading2".into()), egui::FontId::proportional(20.0)),
        (egui::TextStyle::Body, egui::FontId::proportional(16.0)),
        (egui::TextStyle::Button, egui::FontId::proportional(14.0)),
        (egui::TextStyle::Small, egui::FontId::proportional(12.0)),
    ].into();
    
    // Spacing
    style.spacing.item_spacing = egui::vec2(10.0, 12.0);
    style.spacing.button_padding = egui::vec2(12.0, 8.0);
    
    // Colors - dark theme
    let mut visuals = egui::Visuals::dark();
    visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(45, 45, 50);
    visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(66, 66, 72);
    visuals.widgets.active.bg_fill = egui::Color32::from_rgb(88, 88, 95);
    visuals.widgets.inactive.rounding = egui::Rounding::same(8.0);
    visuals.widgets.hovered.rounding = egui::Rounding::same(8.0);
    visuals.widgets.active.rounding = egui::Rounding::same(8.0);
    
    ctx.set_visuals(visuals);
    ctx.set_style(style);
}
