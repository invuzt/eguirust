use eframe::egui;
use egui_plot::{Legend, Line, Plot, PlotPoints, Points};

pub struct PlotApp {
    time: f64,
}

impl Default for PlotApp {
    fn default() -> Self {
        Self {
            time: 0.0,
        }
    }
}

impl PlotApp {
    fn generate_sensor_data(&self) -> (PlotPoints, PlotPoints, PlotPoints) {
        let mut temp_data = Vec::new();
        let mut pressure_data = Vec::new();
        let mut fps_data = Vec::new();
        
        for i in 0..200 {
            let x = i as f64 * 0.1;
            temp_data.push([x, 20.0 + 5.0 * (x * 0.5).sin() + (self.time * 0.5).sin()]);
            pressure_data.push([x, 1013.0 + 10.0 * (x * 0.3).cos()]);
            fps_data.push([x, 60.0 + 5.0 * (x * 0.8).sin() + 2.0 * (self.time).sin()]);
        }
        
        (
            PlotPoints::new(temp_data),
            PlotPoints::new(pressure_data),
            PlotPoints::new(fps_data),
        )
    }
    
    fn generate_scatter_data(&self) -> (PlotPoints, PlotPoints) {
        let mut random_cluster = Vec::new();
        let mut normal_cluster = Vec::new();
        
        for i in 0..100 {
            let angle = i as f64 * 0.1;
            random_cluster.push([angle.cos() * 2.0, angle.sin() * 2.0]);
            normal_cluster.push([angle.cos() * 4.0 + 2.0, angle.sin() * 4.0 + 2.0]);
        }
        
        (
            PlotPoints::new(random_cluster),
            PlotPoints::new(normal_cluster),
        )
    }
}

impl eframe::App for PlotApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.time += 0.016;
        
        let screen_rect = ctx.input(|i| i.screen_rect());
        let screen_height = screen_rect.height();
        
        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            ui.add_space(screen_height * 0.05);
            ui.horizontal(|ui| {
                ui.add_space(12.0);
                ui.label(format!("Real-time Sensor Data | Time: {:.2}s", self.time));
            });
            ui.add_space(8.0);
        });
        
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(10.0);
            
            // TIME SERIES PLOT (Line Chart)
            let (temp_data, pressure_data, fps_data) = self.generate_sensor_data();
            
            let temp_line = Line::new(temp_data)
                .color(egui::Color32::from_rgb(255, 100, 100))
                .name("Temperature (C)")
                .width(2.0);
            
            let pressure_line = Line::new(pressure_data)
                .color(egui::Color32::from_rgb(100, 100, 255))
                .name("Pressure (hPa)")
                .width(2.0);
            
            let fps_line = Line::new(fps_data)
                .color(egui::Color32::from_rgb(100, 255, 100))
                .name("FPS")
                .width(2.0);
            
            Plot::new("time_series")
                .height(350.0)
                .allow_zoom(true)
                .allow_drag(true)
                .show_axes([true, true])
                .show_grid(true)
                .legend(Legend::default())
                .x_axis_label("Time (seconds)")
                .y_axis_label("Values")
                .show(ui, |plot_ui| {
                    plot_ui.line(temp_line);
                    plot_ui.line(pressure_line);
                    plot_ui.line(fps_line);
                });
            
            ui.add_space(20.0);
            
            // SCATTER PLOT (Points)
            let (cluster_a, cluster_b) = self.generate_scatter_data();
            
            let points1 = Points::new(cluster_a)
                .color(egui::Color32::from_rgb(100, 200, 255))
                .name("Cluster A")
                .radius(4.0)
                .filled(true);
            
            let points2 = Points::new(cluster_b)
                .color(egui::Color32::from_rgb(255, 200, 100))
                .name("Cluster B")
                .radius(4.0)
                .filled(true)
                .shape(egui_plot::MarkerShape::Diamond);
            
            Plot::new("scatter")
                .height(350.0)
                .allow_zoom(true)
                .allow_drag(true)
                .show_axes([true, true])
                .show_grid(true)
                .legend(Legend::default())
                .x_axis_label("X Position")
                .y_axis_label("Y Position")
                .show(ui, |plot_ui| {
                    plot_ui.points(points1);
                    plot_ui.points(points2);
                });
            
            ui.label("Top: Time series (temperature, pressure, FPS) | Bottom: Scatter plot with two clusters");
            
            ui.add_space(20.0);
            
            egui::Frame::none()
                .fill(egui::Color32::from_rgb(30, 30, 35))
                .rounding(egui::Rounding::same(8.0))
                .inner_margin(egui::Margin::same(12.0))
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label("Features: Zoom/Pan | Multiple Y-Axis | Legend | Real-time Update");
                    });
                });
        });
    }
}
