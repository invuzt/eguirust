use eframe::egui;

pub struct Agent {
    pub name: String,
    pub pos: egui::Pos2,
    pub color: egui::Color32,
}

pub struct Connection {
    pub from: usize,
    pub to: usize,
}

pub struct AppState {
    pub agents: Vec<Agent>,
    pub connections: Vec<Connection>,
    pub selected_agent: Option<usize>,
    pub show_kb: bool,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            agents: Vec::new(),
            connections: Vec::new(),
            selected_agent: None,
            show_kb: false,
        }
    }

    pub fn add_agent(&mut self) {
        let id = self.agents.len();
        let colors = [
            egui::Color32::from_rgb(0, 150, 255), // Biru
            egui::Color32::from_rgb(0, 255, 150), // Hijau
            egui::Color32::from_rgb(255, 150, 0), // Oranye
        ];
        
        let new_agent = Agent {
            name: format!("Agent {}", id),
            pos: egui::pos2(100.0, 200.0 + (id as f32 * 20.0)),
            color: colors[id % colors.len()],
        };
        
        self.agents.push(new_agent);
        
        // Logika Auto-Connect: Sambungkan ke agent sebelumnya jika ada
        if id > 0 {
            self.connections.push(Connection { from: id - 1, to: id });
        }
    }
}
