use eframe::egui;

pub struct Agent {
    pub name: String,
    pub pos: egui::Pos2,
    pub color: egui::Color32,
}

pub struct Connection {
    pub from: usize,
    pub to: usize,
    pub message: String, // Pesan yang dibawa titik data
}

pub struct AppState {
    pub agents: Vec<Agent>,
    pub connections: Vec<Connection>,
    pub selected_agent: Option<usize>,
    pub show_kb: bool,
    pub is_running: bool,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            agents: Vec::new(),
            connections: Vec::new(),
            selected_agent: None,
            show_kb: false,
            is_running: false,
        }
    }

    pub fn add_agent(&mut self) {
        let id = self.agents.len();
        let colors = [
            egui::Color32::from_rgb(0, 150, 255),
            egui::Color32::from_rgb(0, 255, 150),
            egui::Color32::from_rgb(200, 100, 255),
        ];
        
        let x = 60.0 + (id as f32 * 40.0 % 240.0);
        let y = 160.0 + (id as f32 * 90.0 % 450.0);

        self.agents.push(Agent {
            name: format!("AGENT_{}", id),
            pos: egui::pos2(x, y),
            color: colors[id % colors.len()],
        });
        
        if id > 0 {
            // Berikan pesan simulasi otomatis antar agent
            let msg = match id {
                1 => "Analyzing...".to_string(),
                2 => "Executing...".to_string(),
                _ => "Passing Data...".to_string(),
            };
            self.connections.push(Connection { from: id - 1, to: id, message: msg });
        }
    }
}
