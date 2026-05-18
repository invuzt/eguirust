use eframe::egui;
use crate::app_logic::{AppState, FieldType};

pub fn render_ui(ctx: &egui::Context, state: &mut AppState) {
    egui::TopBottomPanel::top("header")
        .frame(egui::Frame::none())
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("VUZT CRUD APP");
                if ui.button("RESET VIEW").clicked() {
                    state.view_offset = egui::vec2(0.0, 0.0);
                    state.zoom_factor = 1.0;
                }
            });
            ui.add_space(10.0);
        });

    egui::CentralPanel::default()
        .frame(egui::Frame::none())
        .show(ctx, |ui| {
            // Gunakan scroll area untuk konten utama
            egui::ScrollArea::vertical()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    ui.add_space(10.0);
                    
                    // FORM SECTION
                    ui.group(|ui| {
                        ui.vertical(|ui| {
                            ui.label(if state.edit_mode { "EDIT ITEM" } else { "CREATE NEW ITEM" });
                            ui.add_space(5.0);
                            
                            ui.horizontal(|ui| {
                                ui.label("Name:");
                                let resp = ui.text_edit_singleline(&mut state.form_name);
                                if resp.clicked() && !state.show_kb {
                                    state.selected_field = FieldType::Name;
                                    state.show_kb = true;
                                }
                            });
                            
                            ui.add_space(5.0);
                            
                            ui.horizontal(|ui| {
                                ui.label("Description:");
                                let resp = ui.text_edit_multiline(&mut state.form_desc);
                                if resp.clicked() && !state.show_kb {
                                    state.selected_field = FieldType::Description;
                                    state.show_kb = true;
                                }
                            });
                            
                            ui.add_space(10.0);
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
                    
                    // STATISTICS SECTION
                    ui.group(|ui| {
                        ui.vertical(|ui| {
                            ui.label(format!("Total Items: {}", state.items.len()));
                            if let Some(idx) = state.selected_item {
                                if let Some(item) = state.items.get(idx) {
                                    ui.separator();
                                    ui.label(format!("Selected ID: {}", item.id));
                                    ui.label(format!("Selected Name: {}", item.name));
                                }
                            }
                        });
                    });
                    
                    ui.add_space(10.0);
                    
                    // ITEMS LIST SECTION
                    ui.group(|ui| {
                        ui.vertical(|ui| {
                            ui.label("ITEMS LIST");
                            ui.add_space(5.0);
                            
                            if state.items.is_empty() {
                                ui.label("No items yet. Create one!");
                            } else {
                                let items_to_display: Vec<(usize, String, String, String)> = state.items
                                    .iter()
                                    .enumerate()
                                    .map(|(i, item)| (i, item.name.clone(), item.description.clone(), item.created_at.clone()))
                                    .collect();
                                
                                for (i, name, description, created_at) in items_to_display {
                                    ui.horizontal(|ui| {
                                        ui.vertical(|ui| {
                                            ui.label(format!("Name: {}", name));
                                            ui.label(format!("Desc: {}", description));
                                            ui.label(format!("Date: {}", created_at));
                                        });
                                        
                                        ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                                            if ui.button("Delete").clicked() {
                                                state.delete_item(i);
                                            }
                                            if ui.button("Edit").clicked() {
                                                state.start_edit(i);
                                                state.show_kb = false;
                                            }
                                            if ui.button("Select").clicked() {
                                                state.selected_item = Some(i);
                                            }
                                        });
                                    });
                                    ui.separator();
                                }
                            }
                        });
                    });
                    
                    ui.add_space(20.0);
                });
        });
}
