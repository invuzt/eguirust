use eframe::egui;
use crate::app_logic::{AppState, ActiveField};

pub fn render_ui(ctx: &egui::Context, state: &mut AppState) {
    egui::TopBottomPanel::top("header")
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("VUZT CRUD APP");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("RESET").clicked() {
                        state.view_offset = egui::vec2(0.0, 0.0);
                        state.zoom_factor = 1.0;
                    }
                });
            });
            ui.add_space(5.0);
        });

    egui::CentralPanel::default()
        .show(ctx, |ui| {
            ui.vertical(|ui| {
                // FORM SECTION
                ui.group(|ui| {
                    ui.vertical(|ui| {
                        ui.label(if state.edit_mode { "EDIT ITEM" } else { "NEW ITEM" });
                        ui.add_space(8.0);
                        
                        // NAME FIELD - click to activate
                        ui.horizontal(|ui| {
                            ui.label("Name:");
                            let resp = ui.text_edit_singleline(&mut state.form_name);
                            if resp.clicked() {
                                state.set_active_field(ActiveField::Name);
                            }
                        });
                        
                        ui.add_space(8.0);
                        
                        // DESCRIPTION FIELD - click to activate
                        ui.horizontal(|ui| {
                            ui.label("Desc:");
                            let resp = ui.text_edit_multiline(&mut state.form_desc);
                            if resp.clicked() {
                                state.set_active_field(ActiveField::Description);
                            }
                        });
                        
                        ui.add_space(12.0);
                        ui.horizontal(|ui| {
                            if state.edit_mode {
                                if ui.button("UPDATE").clicked() {
                                    state.update_item();
                                    state.show_kb = false;
                                }
                                if ui.button("CANCEL").clicked() {
                                    state.cancel_edit();
                                    state.show_kb = false;
                                }
                            } else {
                                if ui.button("SAVE").clicked() {
                                    state.create_item();
                                    state.show_kb = false;
                                }
                            }
                        });
                    });
                });
                
                ui.add_space(10.0);
                
                // STATISTICS
                ui.group(|ui| {
                    ui.vertical(|ui| {
                        ui.label(format!("Total: {}", state.items.len()));
                        if let Some(idx) = state.selected_item {
                            if let Some(item) = state.items.get(idx) {
                                ui.separator();
                                ui.label(format!("Selected ID: {}", item.id));
                                ui.label(format!("Name: {}", item.name));
                            }
                        }
                    });
                });
                
                ui.add_space(10.0);
                
                // ITEMS LIST
                ui.group(|ui| {
                    ui.vertical(|ui| {
                        ui.label("ITEMS LIST");
                        ui.add_space(5.0);
                        
                        if state.items.is_empty() {
                            ui.label("No items");
                        } else {
                            egui::ScrollArea::vertical()
                                .max_height(350.0)
                                .show(ui, |ui| {
                                    let items_clone: Vec<(usize, String, String, String)> = state.items
                                        .iter()
                                        .enumerate()
                                        .map(|(i, item)| (i, item.name.clone(), item.description.clone(), item.created_at.clone()))
                                        .collect();
                                    
                                    for (i, name, desc, date) in items_clone {
                                        ui.horizontal(|ui| {
                                            ui.vertical(|ui| {
                                                ui.label(format!("{}", name));
                                                ui.label(format!("{}", desc));
                                            });
                                            
                                            ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                                                if ui.button("Del").clicked() {
                                                    state.delete_item(i);
                                                }
                                                if ui.button("Edit").clicked() {
                                                    state.start_edit(i);
                                                }
                                                if ui.button("Sel").clicked() {
                                                    state.selected_item = Some(i);
                                                }
                                            });
                                        });
                                        ui.separator();
                                    }
                                });
                        }
                    });
                });
                
                ui.add_space(20.0);
            });
        });
}
