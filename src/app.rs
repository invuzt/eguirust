use eframe::egui;
use egui_plot::{Legend, Line, Plot, PlotPoints, Points};
use rand::Rng;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

#[derive(Clone, Debug)]
struct WeatherData {
    temperature: f64,
    humidity: f64,
    wind_speed: f64,
    timestamp: f64,
}

#[derive(Clone, Debug)]
struct CryptoData {
    price: f64,
    volume: f64,
    timestamp: f64,
}

pub struct DashboardApp {
    weather_history: Vec<WeatherData>,
    crypto_history: Vec<CryptoData>,
    btc_price: f64,
    eth_price: f64,
    random_values: Vec<[f64; 2]>,
    time: f64,
    data_source: usize,
    receiver: mpsc::Receiver<String>,
}

impl Default for DashboardApp {
    fn default() -> Self {
        let (tx, rx) = mpsc::channel();
        
        // Start background thread for API calls
        thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                let client = reqwest::Client::new();
                let mut btc_price = 50000.0;
                
                loop {
                    // Get Bitcoin price from Binance
                    let btc_url = "https://api.binance.com/api/v3/ticker/price?symbol=BTCUSDT";
                    if let Ok(response) = client.get(btc_url).send().await {
                        if let Ok(json) = response.json::<serde_json::Value>().await {
                            if let Some(price) = json["price"].as_str() {
                                if let Ok(p) = price.parse::<f64>() {
                                    btc_price = p;
                                    let _ = tx.send(format!("BTC: {}", p));
                                }
                            }
                        }
                    }
                    
                    // Get weather from Open-Meteo (Jakarta)
                    let weather_url = "https://api.open-meteo.com/v1/forecast?latitude=-6.21&longitude=106.85&current_weather=true";
                    if let Ok(response) = client.get(weather_url).send().await {
                        if let Ok(json) = response.json::<serde_json::Value>().await {
                            if let Some(temp) = json["current_weather"]["temperature"].as_f64() {
                                let _ = tx.send(format!("Temp: {} C", temp));
                            }
                        }
                    }
                    
                    thread::sleep(Duration::from_secs(10));
                }
            });
        });
        
        let mut app = Self {
            weather_history: Vec::new(),
            crypto_history: Vec::new(),
            btc_price: 50000.0,
            eth_price: 3000.0,
            random_values: Vec::new(),
            time: 0.0,
            data_source: 0,
            receiver: rx,
        };
        
        // Initialize random data
        let mut rng = rand::thread_rng();
        for i in 0..100 {
            app.random_values.push([i as f64, rng.gen_range(0.0..100.0)]);
        }
        
        app
    }
}

impl DashboardApp {
    fn update_from_api(&mut self) {
        while let Ok(msg) = self.receiver.try_recv() {
            log::info!("API: {}", msg);
        }
    }
    
    fn generate_weather_data(&mut self) -> PlotPoints {
        let mut points = Vec::new();
        let mut rng = rand::thread_rng();
        
        for i in 0..100 {
            let time = self.time - (100 - i) as f64 * 0.1;
            let temp = 25.0 + 5.0 * (time * 0.1).sin() + rng.gen_range(-1.0..1.0);
            points.push([time, temp]);
        }
        
        PlotPoints::new(points)
    }
    
    fn generate_crypto_data(&mut self) -> (PlotPoints, PlotPoints) {
        let mut btc_points = Vec::new();
        let mut eth_points = Vec::new();
        let mut rng = rand::thread_rng();
        
        for i in 0..100 {
            let time = self.time - (100 - i) as f64 * 0.1;
            let btc = 50000.0 + 5000.0 * (time * 0.5).sin() + rng.gen_range(-200.0..200.0);
            let eth = 3000.0 + 300.0 * (time * 0.6).cos() + rng.gen_range(-50.0..50.0);
            btc_points.push([time, btc]);
            eth_points.push([time, eth]);
        }
        
        (PlotPoints::new(btc_points), PlotPoints::new(eth_points))
    }
    
    fn generate_scatter_data(&self) -> (PlotPoints, PlotPoints, PlotPoints) {
        let mut rng = rand::thread_rng();
        let mut cluster1 = Vec::new();
        let mut cluster2 = Vec::new();
        let mut cluster3 = Vec::new();
        
        for _ in 0..50 {
            cluster1.push([rng.gen_range(0.0..3.0), rng.gen_range(0.0..3.0)]);
            cluster2.push([rng.gen_range(4.0..7.0), rng.gen_range(4.0..7.0)]);
            cluster3.push([rng.gen_range(8.0..11.0), rng.gen_range(2.0..5.0)]);
        }
        
        (
            PlotPoints::new(cluster1),
            PlotPoints::new(cluster2),
            PlotPoints::new(cluster3),
        )
    }
}

impl eframe::App for DashboardApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.time += 0.016;
        self.update_from_api();
        
        let screen_rect = ctx.input(|i| i.screen_rect());
        let screen_height = screen_rect.height();
        
        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            ui.add_space(screen_height * 0.05);
            ui.horizontal(|ui| {
                ui.add_space(12.0);
                
                if ui.button("Weather").clicked() {
                    self.data_source = 0;
                }
                if ui.button("Crypto").clicked() {
                    self.data_source = 1;
                }
                if ui.button("Scatter").clicked() {
                    self.data_source = 2;
                }
                if ui.button("Random").clicked() {
                    self.data_source = 3;
                }
                
                ui.add_space(20.0);
                ui.label(format!("BTC: ${:.0} | ETH: ${:.0}", self.btc_price, self.eth_price));
                ui.add_space(10.0);
                ui.label(format!("Time: {:.1}s", self.time));
            });
            ui.add_space(8.0);
        });
        
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(10.0);
            
            match self.data_source {
                0 => {
                    // WEATHER PLOT
                    let temp_data = self.generate_weather_data();
                    let temp_line = Line::new(temp_data)
                        .color(egui::Color32::from_rgb(255, 100, 100))
                        .name("Temperature (C)")
                        .width(2.0);
                    
                    Plot::new("weather")
                        .height(500.0)
                        .allow_zoom(true)
                        .allow_drag(true)
                        .show_axes([true, true])
                        .show_grid(true)
                        .legend(Legend::default())
                        .x_axis_label("Time (minutes)")
                        .y_axis_label("Temperature (C)")
                        .show(ui, |plot_ui| {
                            plot_ui.line(temp_line);
                        });
                    
                    ui.label("Weather Data: Temperature simulation with random noise");
                }
                
                1 => {
                    // CRYPTO PLOT
                    let (btc_data, eth_data) = self.generate_crypto_data();
                    
                    let btc_line = Line::new(btc_data)
                        .color(egui::Color32::from_rgb(255, 200, 50))
                        .name("BTC/USDT")
                        .width(2.0);
                    
                    let eth_line = Line::new(eth_data)
                        .color(egui::Color32::from_rgb(100, 200, 255))
                        .name("ETH/USDT")
                        .width(2.0);
                    
                    Plot::new("crypto")
                        .height(500.0)
                        .allow_zoom(true)
                        .allow_drag(true)
                        .show_axes([true, true])
                        .show_grid(true)
                        .legend(Legend::default())
                        .x_axis_label("Time (minutes)")
                        .y_axis_label("Price (USD)")
                        .show(ui, |plot_ui| {
                            plot_ui.line(btc_line);
                            plot_ui.line(eth_line);
                        });
                    
                    ui.label("Crypto Data: Bitcoin and Ethereum price simulation");
                }
                
                2 => {
                    // SCATTER PLOT
                    let (cluster1, cluster2, cluster3) = self.generate_scatter_data();
                    
                    let points1 = Points::new(cluster1)
                        .color(egui::Color32::from_rgb(100, 200, 255))
                        .name("Cluster A")
                        .radius(5.0)
                        .filled(true);
                    
                    let points2 = Points::new(cluster2)
                        .color(egui::Color32::from_rgb(255, 100, 100))
                        .name("Cluster B")
                        .radius(5.0)
                        .filled(true)
                        .shape(egui_plot::MarkerShape::Diamond);
                    
                    let points3 = Points::new(cluster3)
                        .color(egui::Color32::from_rgb(100, 255, 100))
                        .name("Cluster C")
                        .radius(5.0)
                        .filled(true)
                        .shape(egui_plot::MarkerShape::Cross);
                    
                    Plot::new("scatter")
                        .height(500.0)
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
                    
                    ui.label("Scatter Plot: Three synthetic data clusters");
                }
                
                _ => {
                    // RANDOM DATA PLOT
                    let random_line = Line::new(PlotPoints::new(self.random_values.clone()))
                        .color(egui::Color32::from_rgb(150, 150, 255))
                        .name("Random Values")
                        .width(2.0);
                    
                    Plot::new("random")
                        .height(500.0)
                        .allow_zoom(true)
                        .allow_drag(true)
                        .show_axes([true, true])
                        .show_grid(true)
                        .legend(Legend::default())
                        .x_axis_label("Index")
                        .y_axis_label("Value")
                        .show(ui, |plot_ui| {
                            plot_ui.line(random_line);
                        });
                    
                    if ui.button("Generate New Random Data").clicked() {
                        let mut rng = rand::thread_rng();
                        self.random_values.clear();
                        for i in 0..100 {
                            self.random_values.push([i as f64, rng.gen_range(0.0..100.0)]);
                        }
                    }
                    
                    ui.label("Random Data Generator: Click button to generate new random values");
                }
            }
            
            ui.add_space(20.0);
            
            egui::Frame::none()
                .fill(egui::Color32::from_rgb(30, 30, 35))
                .rounding(egui::Rounding::same(8.0))
                .inner_margin(egui::Margin::same(12.0))
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label("Data Sources: Weather (simulated) | Crypto (simulated) | Scatter (synthetic) | Random Generator");
                    });
                    ui.horizontal(|ui| {
                        ui.label("Features: Zoom/Pan | Real-time | Multiple datasets | Interactive");
                    });
                });
        });
    }
}
