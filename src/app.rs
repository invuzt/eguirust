use eframe::egui;

#[derive(Clone)]
struct Card {
    title: String,
    value: String,
    color: egui::Color32,
    size: CardSize,
}

#[derive(Clone)]
enum CardSize {
    Small,
    Medium,
    Large,
    Wide,
}

pub struct MyApp {
    cards: Vec<Card>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            cards: vec![
                Card { title: "Revenue".to_string(), value: "$12,450".to_string(), color: egui::Color32::from_rgb(34, 197, 94), size: CardSize::Medium },
                Card { title: "Users".to_string(), value: "2,847".to_string(), color: egui::Color32::from_rgb(59, 130, 246), size: CardSize::Small },
                Card { title: "Growth".to_string(), value: "+23%".to_string(), color: egui::Color32::from_rgb(168, 85, 247), size: CardSize::Small },
                Card { title: "Active".to_string(), value: "1,234".to_string(), color: egui::Color32::from_rgb(239, 68, 68), size: CardSize::Medium },
                Card { title: "Chart".to_string(), value: "📊 Weekly".to_string(), color: egui::Color32::from_rgb(20, 150, 150), size: CardSize::Wide },
                Card { title: "Tasks".to_string(), value: "12/24".to_string(), color: egui::Color32::from_rgb(250, 150, 50), size: CardSize::Small },
                Card { title: "Messages".to_string(), value: "5 new".to_string(), color: egui::Color32::from_rgb(100, 100, 200), size: CardSize::Large },
                Card { title: "Storage".to_string(), value: "64%".to_string(), color: egui::Color32::from_rgb(200, 100, 100), size: CardSize::Medium },
                Card { title: "Analytics".to_string(), value: "+45%".to_string(), color: egui::Color32::from_rgb(50, 180, 120), size: CardSize::Small },
                Card { title: "Sales".to_string(), value: "$8,230".to_string(), color: egui::Color32::from_rgb(220, 120, 80), size: CardSize::Medium },
                Card { title: "Traffic".to_string(), value: "12.4k".to_string(), color: egui::Color32::from_rgb(80, 140, 220), size: CardSize::Small },
            ],
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let screen_rect = ctx.input(|i| i.screen_rect());
        let screen_width = screen_rect.width();
        let screen_height = screen_rect.height();
        
        let padding = 12.0;
        let gap = 12.0;
        let cols = if screen_width < 400.0 { 2 } else if screen_width < 700.0 { 3 } else { 4 };
        let card_base_width = (screen_width - (padding * 2.0) - (gap * (cols as f32 - 1.0))) / cols as f32;
        
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(screen_height * 0.05);
            
            ui.vertical_centered(|ui| {
                ui.heading(egui::RichText::new("Bento Dashboard").size(28.0).color(egui::Color32::WHITE));
                ui.label(egui::RichText::new("Responsive masonry layout - wraps vertically").size(14.0).color(egui::Color32::from_gray(150)));
                ui.add_space(20.0);
            });
            
            egui::ScrollArea::vertical()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    ui.add_space(8.0);
                    
                    // Use horizontal_wrapped untuk wrap ke bawah otomatis
                    let mut x_offset = 0.0;
                    let mut row_height = 0.0;
                    let mut current_row_cards: Vec<(usize, f32, f32)> = Vec::new();
                    
                    // Layout calculation untuk masonry wrap
                    for (idx, card) in self.cards.iter().enumerate() {
                        let card_width = match card.size {
                            CardSize::Small => card_base_width,
                            CardSize::Medium => card_base_width * 1.5,
                            CardSize::Large => card_base_width * 2.0,
                            CardSize::Wide => card_base_width * 2.5,
                        };
                        
                        let card_height = match card.size {
                            CardSize::Small => card_width * 0.6,
                            CardSize::Medium => card_width * 0.7,
                            CardSize::Large => card_width * 0.8,
                            CardSize::Wide => card_width * 0.5,
                        };
                        
                        if x_offset + card_width > screen_width - padding * 2.0 {
                            // Turun ke baris baru
                            x_offset = 0.0;
                            row_height = 0.0;
                        }
                        
                        current_row_cards.push((idx, x_offset, card_height));
                        x_offset += card_width + gap;
                        row_height = row_height.max(card_height);
                    }
                    
                    // Render dengan layout wrap
                    let mut current_x = 0.0;
                    let mut current_y = 0.0;
                    let mut row_max_height = 0.0;
                    
                    for (idx, card) in self.cards.iter_mut().enumerate() {
                        let card_width = match card.size {
                            CardSize::Small => card_base_width,
                            CardSize::Medium => card_base_width * 1.5,
                            CardSize::Large => card_base_width * 2.0,
                            CardSize::Wide => card_base_width * 2.5,
                        };
                        
                        let card_height = match card.size {
                            CardSize::Small => card_width * 0.6,
                            CardSize::Medium => card_width * 0.7,
                            CardSize::Large => card_width * 0.8,
                            CardSize::Wide => card_width * 0.5,
                        };
                        
                        if current_x + card_width > screen_width - padding * 2.0 {
                            current_x = 0.0;
                            current_y += row_max_height + gap;
                            row_max_height = 0.0;
                        }
                        
                        let frame = egui::Frame::none()
                            .fill(card.color)
                            .rounding(egui::Rounding::same(16.0))
                            .inner_margin(egui::Margin::same(12.0));
                        
                        // Position using absolute layout untuk wrap yang presisi
                        ui.allocate_ui_at_rect(
                            egui::Rect::from_min_size(
                                egui::pos2(current_x + padding, current_y + 80.0),
                                egui::vec2(card_width, card_height),
                            ),
                            |ui| {
                                frame.show(ui, |ui| {
                                    ui.set_min_size(egui::vec2(card_width, card_height));
                                    ui.vertical(|ui| {
                                        ui.label(egui::RichText::new(&card.title).size(14.0).color(egui::Color32::WHITE).weak());
                                        ui.add_space(8.0);
                                        ui.label(egui::RichText::new(&card.value).size(24.0).color(egui::Color32::WHITE).strong());
                                        ui.add_space(4.0);
                                        
                                        match idx % 4 {
                                            0 => { ui.label(egui::RichText::new("↑ +12%").size(11.0).color(egui::Color32::from_rgb(150, 255, 150))); }
                                            1 => { ui.label(egui::RichText::new("↓ -3%").size(11.0).color(egui::Color32::from_rgb(255, 150, 150))); }
                                            _ => { ui.label(egui::RichText::new("→ stable").size(11.0).color(egui::Color32::from_gray(180))); }
                                        }
                                    });
                                });
                            },
                        );
                        
                        current_x += card_width + gap;
                        row_max_height = row_max_height.max(card_height);
                    }
                    
                    let last_y = current_y + row_max_height + 120.0;
                    
                    // Full width summary card di bagian bawah
                    ui.add_space(last_y - 80.0);
                    ui.add_space(20.0);
                    
                    ui.horizontal_wrapped(|ui| {
                        let full_width = screen_width - padding * 2.0;
                        let full_frame = egui::Frame::none()
                            .fill(egui::Color32::from_rgb(40, 40, 48))
                            .rounding(egui::Rounding::same(16.0))
                            .inner_margin(egui::Margin::same(16.0));
                        
                        full_frame.show(ui, |ui| {
                            ui.set_min_size(egui::vec2(full_width, 100.0));
                            ui.horizontal(|ui| {
                                ui.label(egui::RichText::new("📈 Summary").size(16.0).color(egui::Color32::WHITE));
                                ui.add_space(20.0);
                                ui.label(egui::RichText::new("Total: $15,297").size(16.0).color(egui::Color32::from_rgb(150, 255, 150)));
                                ui.add_space(20.0);
                                ui.label(egui::RichText::new("Items: 2,847").size(16.0).color(egui::Color32::from_rgb(150, 200, 255)));
                                ui.add_space(20.0);
                                ui.label(egui::RichText::new("Growth: +23%").size(16.0).color(egui::Color32::from_rgb(255, 200, 100)));
                            });
                        });
                    });
                    
                    ui.add_space(40.0);
                });
        });
    }
}
