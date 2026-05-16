use egui::Context;
use egui::plot::{Legend, Line, Plot, PlotPoints, Points};
use rand::Rng;

pub struct GraphApp {
    weather_history: Vec<[f64; 2]>,
    crypto_history: Vec<[f64; 2]>,
    crypto_history2: Vec<[f64; 2]>,
    random_values: Vec<[f64; 2]>,
    time: f64,
    data_source: usize,
    btc_price: f64,
    eth_price: f64,
}

impl Default for GraphApp {
    fn default() -> Self {
        let mut app = Self {
            weather_history: Vec::new(),
            crypto_history: Vec::new(),
            crypto_history2: Vec::new(),
            random_values: Vec::new(),
            time: 0.0,
            data_source: 0,
            btc_price: 50000.0,
            eth_price: 3000.0,
        };
        
        for i in 0..100 {
            let x = i as f64 * 0.1;
            app.weather_history.push([x, 25.0 + 5.0 * (x * 0.3).sin()]);
            app.crypto_history.push([x, 50000.0 + 5000.0 * (x * 0.2).sin()]);
            app.crypto_history2.push([x, 3000.0 + 500.0 * (x * 0.25).cos()]);
            app.random_values.push([x, (i % 100) as f64]);
        }
        
        app
    }
}

impl GraphApp {
    pub fn update(&mut self, ctx: &Context) {
        self.update_data();
        
        let screen_rect = ctx.input(|i| i.screen_rect());
        let screen_width = screen_rect.width();
        let screen_height = screen_rect.height();
        
        let is_portrait = screen_height > screen_width;
        let top_padding = if is_portrait { screen_height * 0.12 } else { screen_height * 0.08 };
        let plot_height = if is_portrait { screen_height * 0.55 } else { screen_height * 0.6 };
        
        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            ui.add_space(top_padding);
            ui.horizontal(|ui| {
                ui.add_space(12.0);
                
                egui::menu::bar(ui, |ui| {
                    ui.menu_button("Data Source", |ui| {
                        if ui.button("Weather (Temperature)").clicked() {
                            self.data_source = 0;
                            ui.close_menu();
                        }
                        if ui.button("Crypto (BTC / ETH)").clicked() {
                            self.data_source = 1;
                            ui.close_menu();
                        }
                        if ui.button("Scatter Plot (Clusters)").clicked() {
                            self.data_source = 2;
                            ui.close_menu();
                        }
                        if ui.button("Random Generator").clicked() {
                            self.data_source = 3;
                            ui.close_menu();
                        }
                    });
                });
                
                ui.add_space(20.0);
                
                let current_label = match self.data_source {
                    0 => "Weather",
                    1 => "Crypto",
                    2 => "Scatter",
                    _ => "Random",
                };
                ui.label(egui::RichText::new(format!("Current: {}", current_label)).size(14.0).color(egui::Color32::from_rgb(100, 200, 255)));
                
                ui.add_space(20.0);
                
                ui.label(egui::RichText::new(format!("BTC: ${:.0}", self.btc_price)).size(13.0).color(egui::Color32::from_rgb(255, 200, 100)));
                ui.add_space(12.0);
                ui.label(egui::RichText::new(format!("ETH: ${:.0}", self.eth_price)).size(13.0).color(egui::Color32::from_rgb(255, 200, 100)));
                ui.add_space(12.0);
                ui.label(egui::RichText::new(format!("Time: {:.1}s", self.time)).size(13.0).color(egui::Color32::from_gray(180)));
            });
            ui.add_space(8.0);
        });
        
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(8.0);
            
            match self.data_source {
                0 => {
                    let temp_points = PlotPoints::new(self.weather_history.clone());
                    let temp_line = Line::new(temp_points)
                        .color(egui::Color32::from_rgb(255, 100, 100))
                        .name("Temperature (C)")
                        .width(2.0);
                    
                    Plot::new("weather")
                        .height(plot_height)
                        .allow_zoom(true)
                        .allow_drag(true)
                        .show_axes([true, true])
                        .show_grid(true)
                        .legend(Legend::default())
                        .x_axis_label("Time (seconds)")
                        .y_axis_label("Temperature (Celsius)")
                        .show(ui, |plot_ui| {
                            plot_ui.line(temp_line);
                        });
                    
                    ui.label(egui::RichText::new("Weather Simulation: Real-time temperature with sine wave pattern").color(egui::Color32::from_gray(180)).size(11.0));
                }
                
                1 => {
                    let btc_points = PlotPoints::new(self.crypto_history.clone());
                    let eth_points = PlotPoints::new(self.crypto_history2.clone());
                    
                    let btc_line = Line::new(btc_points)
                        .color(egui::Color32::from_rgb(255, 200, 50))
                        .name("Bitcoin (BTC)")
                        .width(2.0);
                    
                    let eth_line = Line::new(eth_points)
                        .color(egui::Color32::from_rgb(100, 200, 255))
                        .name("Ethereum (ETH)")
                        .width(2.0);
                    
                    Plot::new("crypto")
                        .height(plot_height)
                        .allow_zoom(true)
                        .allow_drag(true)
                        .show_axes([true, true])
                        .show_grid(true)
                        .legend(Legend::default())
                        .x_axis_label("Time (seconds)")
                        .y_axis_label("Price (USD)")
                        .show(ui, |plot_ui| {
                            plot_ui.line(btc_line);
                            plot_ui.line(eth_line);
                        });
                    
                    ui.label(egui::RichText::new("Crypto Simulation: Bitcoin and Ethereum price with market-like volatility").color(egui::Color32::from_gray(180)).size(11.0));
                }
                
                2 => {
                    let (cluster1, cluster2, cluster3) = self.generate_scatter_data();
                    
                    let points1 = Points::new(cluster1)
                        .color(egui::Color32::from_rgb(100, 200, 255))
                        .name("Cluster Alpha")
                        .radius(5.0)
                        .filled(true);
                    
                    let points2 = Points::new(cluster2)
                        .color(egui::Color32::from_rgb(255, 100, 100))
                        .name("Cluster Beta")
                        .radius(5.0)
                        .filled(true)
                        .shape(egui_plot::MarkerShape::Diamond);
                    
                    let points3 = Points::new(cluster3)
                        .color(egui::Color32::from_rgb(100, 255, 100))
                        .name("Cluster Gamma")
                        .radius(5.0)
                        .filled(true)
                        .shape(egui_plot::MarkerShape::Cross);
                    
                    Plot::new("scatter")
                        .height(plot_height)
                        .allow_zoom(true)
                        .allow_drag(true)
                        .show_axes([true, true])
                        .show_grid(true)
                        .legend(Legend::default())
                        .x_axis_label("X Coordinate")
                        .y_axis_label("Y Coordinate")
                        .show(ui, |plot_ui| {
                            plot_ui.points(points1);
                            plot_ui.points(points2);
                            plot_ui.points(points3);
                        });
                    
                    ui.label(egui::RichText::new("Scatter Plot: Three distinct synthetic data clusters").color(egui::Color32::from_gray(180)).size(11.0));
                }
                
                _ => {
                    let mut rng = rand::thread_rng();
                    let random_points: Vec<[f64; 2]> = (0..100).map(|i| [i as f64, rng.gen_range(0.0..100.0)]).collect();
                    let random_line = Line::new(PlotPoints::new(random_points))
                        .color(egui::Color32::from_rgb(150, 150, 255))
                        .name("Random Values")
                        .width(2.0);
                    
                    Plot::new("random")
                        .height(plot_height)
                        .allow_zoom(true)
                        .allow_drag(true)
                        .show_axes([true, true])
                        .show_grid(true)
                        .legend(Legend::default())
                        .x_axis_label("Sample Index")
                        .y_axis_label("Random Value")
                        .show(ui, |plot_ui| {
                            plot_ui.line(random_line);
                        });
                    
                    ui.horizontal(|ui| {
                        if ui.button("Generate New Data").clicked() {
                            ctx.request_repaint();
                        }
                        ui.label(egui::RichText::new("Click to generate fresh random data").color(egui::Color32::from_gray(180)).size(11.0));
                    });
                }
            }
            
            ui.add_space(12.0);
            
            egui::Frame::none()
                .fill(egui::Color32::from_rgb(30, 30, 35))
                .rounding(egui::Rounding::same(8.0))
                .inner_margin(egui::Margin::same(8.0))
                .show(ui, |ui| {
                    if is_portrait {
                        ui.label(egui::RichText::new("Pinch to zoom | Drag to pan").color(egui::Color32::from_gray(180)).size(10.0));
                    } else {
                        ui.horizontal(|ui| {
                            ui.label("Zoom/Pan | Real-time Animation | Legend | Interactive");
                        });
                    }
                });
            
            ui.add_space(20.0);
        });
    }
    
    fn update_data(&mut self) {
        self.time += 0.016;
        
        if self.weather_history.len() > 200 {
            self.weather_history.remove(0);
        }
        let new_temp = 25.0 + 5.0 * (self.time * 0.5).sin() + (self.time * 0.2).cos() * 2.0;
        self.weather_history.push([self.time, new_temp]);
        
        if self.crypto_history.len() > 200 {
            self.crypto_history.remove(0);
            self.crypto_history2.remove(0);
        }
        let new_btc = 50000.0 + 3000.0 * (self.time * 0.3).sin() + (self.time * 0.1).sin() * 1000.0;
        let new_eth = 3000.0 + 400.0 * (self.time * 0.35).cos() + (self.time * 0.15).sin() * 100.0;
        self.crypto_history.push([self.time, new_btc]);
        self.crypto_history2.push([self.time, new_eth]);
        self.btc_price = new_btc;
        self.eth_price = new_eth;
    }
    
    fn generate_scatter_data(&self) -> (PlotPoints, PlotPoints, PlotPoints) {
        let mut rng = rand::thread_rng();
        let mut cluster1 = Vec::new();
        let mut cluster2 = Vec::new();
        let mut cluster3 = Vec::new();
        
        for _ in 0..50 {
            cluster1.push([rng.gen_range(0.0..3.0), rng.gen_range(0.0..3.0)]);
            cluster2.push([rng.gen_range(5.0..8.0), rng.gen_range(5.0..8.0)]);
            cluster3.push([rng.gen_range(9.0..12.0), rng.gen_range(2.0..5.0)]);
        }
        
        (
            PlotPoints::new(cluster1),
            PlotPoints::new(cluster2),
            PlotPoints::new(cluster3),
        )
    }
}
