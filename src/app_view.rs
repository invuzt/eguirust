use eframe::egui;
use crate::app_logic::AppState;

fn primary_button_style(ui: &mut egui::Ui, label: &str) -> egui::Response {
    ui.add_sized(
        [200.0, 50.0],
        egui::Button::new(egui::RichText::new(label).strong())
            .fill(egui::Color32::from_rgb(40, 120, 250))
            .rounding(10.0)
    )
}

pub fn render_ui(ctx: &egui::Context, state: &mut AppState) {
    ctx.set_visuals(egui::Visuals::dark());
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.add_space(60.0);
        ui.vertical_centered(|ui| {
            ui.heading(&state.app_name);
            ui.add_space(20.0);
            egui::Frame::none()
                .fill(egui::Color32::from_gray(30))
                .rounding(15.0)
                .inner_margin(20.0)
                .show(ui, |ui| {
                    ui.label(format!("Nodes Active: {}", state.nodes.len()));
                    ui.add_space(15.0);
                    if primary_button_style(ui, " ➕ ADD NODE").clicked() {
                        state.add_node();
                    }
                });
        });
    });
}
