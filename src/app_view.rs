use eframe::egui;
use crate::app_logic::AppState;

fn primary_button_style(ui: &mut egui::Ui, label: &str) -> egui::Response {
    ui.add_sized(
        [200.0, 50.0],
        egui::Button::new(egui::RichText::new(label).strong())
            .fill(egui::Color32::from_rgb(60, 130, 240))
            .rounding(10.0)
    )
}

pub fn render_ui(ctx: &egui::Context, state: &mut AppState) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.add_space(40.0);
        ui.vertical_centered(|ui| {
            ui.heading(egui::RichText::new(&state.app_name).size(32.0).strong());
            ui.add_space(20.0);
            
            egui::Frame::none()
                .fill(ui.visuals().widgets.inactive.bg_fill)
                .rounding(15.0)
                .inner_margin(20.0)
                .show(ui, |ui| {
                    ui.label(format!("Nodes Connected: {}", state.nodes.len()));
                    ui.add_space(15.0);
                    
                    let resp = ui.add(egui::SelectableLabel::new(
                        state.show_kb,
                        format!(" > Edit Name: {} ", state.app_name)
                    ));
                    if resp.clicked() {
                        state.show_kb = !state.show_kb;
                    }

                    ui.add_space(10.0);
                    if primary_button_style(ui, "➕ ADD NODE").clicked() {
                        state.add_node();
                    }
                });
        });
    });
}
