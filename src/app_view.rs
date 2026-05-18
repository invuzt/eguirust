use eframe::egui;
use crate::app_logic::AppState;

pub fn render_ui(ctx: &egui::Context, state: &mut AppState) {
    egui::TopBottomPanel::top("header")
        .frame(egui::Frame::none().fill(egui::Color32::from_rgb(10, 10, 10)))
        .show(ctx, |ui| {
            ui.add_space(55.0);
            ui.horizontal(|ui| {
                ui.heading("VUZT CRUD APP");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("RESET VIEW").clicked() {
                        state.view_offset = egui::vec2(0.0, 0.0);
                        state.zoom_factor = 1.0;
                    }
                });
            });
        });

    egui::CentralPanel::default()
        .frame(egui::Frame::none().fill(egui::Color32::from_rgb(20, 20, 20)))
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                // LEFT PANEL - Form CRUD
                ui.vertical()
                    .constrained()
                    .min_width(300.0)
                    .max_width(400.0)
                    .show(ui, |ui| {
                        ui.add_space(10.0);
                        ui.group(|ui| {
                            ui.heading(if state.edit_mode { "✏️ EDIT ITEM" } else { "➕ CREATE NEW ITEM" });
                            ui.add_space(10.0);
                            
                            ui.label("📝 NAME:");
                            let resp = ui.text_edit_singleline(&mut state.form_name);
                            if resp.clicked() {
                                state.show_kb = true;
                            }
                            
                            ui.add_space(5.0);
                            ui.label("📄 DESCRIPTION:");
                            let resp = ui.text_edit_multiline(&mut state.form_desc);
                            if resp.clicked() {
                                state.show_kb = true;
                            }
                            
                            ui.add_space(10.0);
                            ui.horizontal(|ui| {
                                if state.edit_mode {
                                    if ui.button("✅ UPDATE").clicked() {
                                        state.update_item();
                                    }
                                    if ui.button("❌ CANCEL").clicked() {
                                        state.cancel_edit();
                                    }
                                } else {
                                    if ui.button("💾 SAVE").clicked() {
                                        state.create_item();
                                    }
                                }
                            });
                        });
                        
                        ui.add_space(10.0);
                        ui.group(|ui| {
                            ui.heading("📊 STATISTICS");
                            ui.label(format!("Total Items: {}", state.items.len()));
                            if let Some(idx) = state.selected_item {
                                if let Some(item) = state.items.get(idx) {
                                    ui.separator();
                                    ui.label("📌 SELECTED:");
                                    ui.label(format!("ID: {}", item.id));
                                    ui.label(format!("Name: {}", item.name));
                                }
                            }
                        });
                    });
                
                ui.separator();
                
                // RIGHT PANEL - List Items
                egui::ScrollArea::vertical()
                    .auto_shrink([false; 2])
                    .show(ui, |ui| {
                        ui.add_space(10.0);
                        ui.heading("📋 ITEMS LIST");
                        ui.add_space(10.0);
                        
                        if state.items.is_empty() {
                            ui.colored_label(egui::Color32::GRAY, "No items yet. Create one!");
                        } else {
                            for (i, item) in state.items.iter().enumerate() {
                                ui.group(|ui| {
                                    ui.horizontal(|ui| {
                                        ui.vertical(|ui| {
                                            ui.label(egui::RichText::new(&item.name).strong());
                                            ui.label(egui::RichText::new(&item.description).weak());
                                            ui.label(egui::RichText::new(format!("📅 {}", item.created_at)).small());
                                        });
                                        
                                        ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                                            if ui.button("🗑️").on_hover_text("Delete").clicked() {
                                                state.delete_item(i);
                                            }
                                            if ui.button("✏️").on_hover_text("Edit").clicked() {
                                                state.start_edit(i);
                                            }
                                            if ui.button("👁️").on_hover_text("Select").clicked() {
                                                state.selected_item = Some(i);
                                            }
                                        });
                                    });
                                });
                                ui.add_space(5.0);
                            }
                        }
                    });
            });
            
            // Handle zoom
            let zoom_delta = ctx.input(|i| i.zoom_delta());
            if zoom_delta != 1.0 {
                state.zoom_factor *= zoom_delta;
                state.zoom_factor = state.zoom_factor.clamp(0.5, 2.0);
            }
        });
}
