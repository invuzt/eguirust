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
        let gap = 8.0;
        let cols = if screen_width < 400.0 { 2 } else if screen_width < 700.0 { 3 } else { 4 };
        let card_base_width = (screen_width - (padding * 2.0) - (gap * (cols as f32 - 1.0))) / cols as f32;
        
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(screen_height * 0.05);
            
            ui.vertical_centered(|ui| {
                ui.heading(egui::RichText::new("Bento Dashboard").size(28.0).color(egui::Color32::WHITE));
                ui.label(egui::RichText::new("Responsive masonry layout").size(14.0).color(egui::Color32::from_gray(150)));
                ui.add_space(20.0);
            });
            
            // Gunakan group dengan background untuk mencegah tumpang tindih
            egui::Frame::none()
                .fill(egui::Color32::from_rgb(18, 18, 22))
                .show(ui, |ui| {
                    egui::ScrollArea::vertical()
                        .auto_shrink([false; 2])
                        .show(ui, |ui| {
                            // Layout dengan grid system
                            let mut row_cards: Vec<Vec<(usize, f32, f32)>> = Vec::new();
                            let mut current_row: Vec<(usize, f32, f32)> = Vec::new();
                            let mut current_row_width: f32 = 0.0;
                            
                            // Kelompokkan ke baris
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
                                
                                if current_row_width + card_width > screen_width - padding * 2.0 {
                                    if !current_row.is_empty() {
                                        row_cards.push(current_row);
                                        current_row = Vec::new();
                                        current_row_width = 0.0;
                                    }
                                }
                                
                                current_row.push((idx, card_width, card_height));
                                current_row_width += card_width + gap;
                            }
                            
                            if !current_row.is_empty() {
                                row_cards.push(current_row);
                            }
                            
                            let mut y_offset: f32 = 0.0;
                            
                            for row in row_cards {
                                let mut row_height: f32 = 0.0;
                                for (_, _, height) in &row {
                                    if *height > row_height {
                                        row_height = *height;
                                    }
                                }
                                
                                let mut x_offset: f32 = padding;
                                for (idx, width, height) in row {
                                    let card = &mut self.cards[idx];
                                    
                                    let frame = egui::Frame::none()
                                        .fill(card.color)
                                        .rounding(egui::Rounding::same(16.0))
                                        .inner_margin(egui::Margin::same(12.0));
                                    
                                    ui.allocate_ui_at_rect(
                                        egui::Rect::from_min_size(
                                            egui::pos2(x_offset, y_offset + 100.0),
                                            egui::vec2(width, height),
                                        ),
                                        |ui| {
                                            frame.show(ui, |ui| {
                                                ui.set_min_size(egui::vec2(width, height));
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
                                    
                                    x_offset += width + gap;
                                }
                                
                                y_offset += row_height + gap;
                            }
                            
                            // Summary card di bawah
                            ui.add_space(y_offset + 20.0);
                            
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
                            
                            ui.add_space(40.0);
                        });
                });
        });
    }
}
