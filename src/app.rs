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
            ],
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let screen_rect = ctx.input(|i| i.screen_rect());
        let screen_width = screen_rect.width();
        let screen_height = screen_rect.height();
        
        // Responsive grid calculation
        let padding = 12.0;
        let gap = 12.0;
        let cols = if screen_width < 400.0 { 2 } else if screen_width < 700.0 { 3 } else { 4 };
        let card_base_width = (screen_width - (padding * 2.0) - (gap * (cols as f32 - 1.0))) / cols as f32;
        
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(screen_height * 0.05);
            
            // Header
            ui.vertical_centered(|ui| {
                ui.heading(egui::RichText::new("Bento Dashboard").size(28.0).color(egui::Color32::WHITE));
                ui.label(egui::RichText::new("Responsive masonry layout").size(14.0).color(egui::Color32::from_gray(150)));
                ui.add_space(20.0);
            });
            
            // Scroll area untuk bento grid
            egui::ScrollArea::vertical()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    ui.add_space(8.0);
                    
                    // Masonry grid menggunakan horizontal_wrapped
                    ui.horizontal_wrapped(|ui| {
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
                            
                            let frame = egui::Frame::none()
                                .fill(card.color)
                                .corner_radius(egui::Rounding::same(16.0))
                                .inner_margin(egui::Margin::same(12.0));
                            
                            frame.show(ui, |ui| {
                                ui.set_min_size(egui::vec2(card_width, card_height));
                                ui.vertical(|ui| {
                                    ui.label(egui::RichText::new(&card.title).size(14.0).color(egui::Color32::WHITE).weak());
                                    ui.add_space(8.0);
                                    ui.label(egui::RichText::new(&card.value).size(24.0).color(egui::Color32::WHITE).strong());
                                    ui.add_space(4.0);
                                    
                                    // Small chart indicator
                                    match idx % 4 {
                                        0 => { ui.label(egui::RichText::new("↑ +12%").size(11.0).color(egui::Color32::from_rgb(150, 255, 150))); }
                                        1 => { ui.label(egui::RichText::new("↓ -3%").size(11.0).color(egui::Color32::from_rgb(255, 150, 150))); }
                                        _ => { ui.label(egui::RichText::new("→ stable").size(11.0).color(egui::Color32::from_gray(180))); }
                                    }
                                });
                            });
                            
                            ui.add_space(gap);
                        }
                    });
                    
                    ui.add_space(20.0);
                    
                    // Full width card
                    ui.horizontal_wrapped(|ui| {
                        let full_width = screen_width - padding * 2.0;
                        let full_frame = egui::Frame::none()
                            .fill(egui::Color32::from_rgb(40, 40, 48))
                            .corner_radius(egui::Rounding::same(16.0))
                            .inner_margin(egui::Margin::same(16.0));
                        
                        full_frame.show(ui, |ui| {
                            ui.set_min_size(egui::vec2(full_width, 100.0));
                            ui.horizontal(|ui| {
                                ui.label(egui::RichText::new("📈 Summary").size(16.0).color(egui::Color32::WHITE));
                                ui.add_space(20.0);
                                ui.label(egui::RichText::new("Total: $15,297").size(16.0).color(egui::Color32::from_rgb(150, 255, 150)));
                                ui.add_space(20.0);
                                ui.label(egui::RichText::new("Items: 2,847").size(16.0).color(egui::Color32::from_rgb(150, 200, 255)));
                            });
                        });
                    });
                    
                    ui.add_space(30.0);
                });
        });
    }
}
