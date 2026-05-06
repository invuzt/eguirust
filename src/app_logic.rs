use eframe::egui;

pub struct Agent {
    pub name: String,
    pub pos: egui::Pos2,
    pub color: egui::Color32,
}

pub struct Connection {
    pub from: usize,
    pub to: usize,
    pub message: String,
}

pub struct AppState {
    pub agents: Vec<Agent>,
    pub connections: Vec<Connection>,
    pub selected_agent: Option<usize>,
    pub link_source: Option<usize>, // Untuk simpan Agent pertama saat buat garis
    pub show_kb: bool,
    pub is_running: bool,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            agents: Vec::new(),
            connections: Vec::new(),
            selected_agent: None,
            link_source: None,
            show_kb: false,
            is_running: false,
        }
    }

    pub fn add_agent(&mut self) {
        let id = self.agents.len();
        self.agents.push(Agent {
            name: format!("AGENT_{}", id),
            pos: egui::pos2(100.0, 200.0),
            color: egui::Color32::from_rgb(0, 120, 255),
        });
    }

    pub fn delete_selected(&mut self) {
        if let Some(idx) = self.selected_agent {
            self.agents.remove(idx);
            // Hapus semua koneksi yang melibatkan agent ini
            self.connections.retain(|c| c.from != idx && c.to != idx);
            // Re-index koneksi yang tersisa agar tidak crash
            for c in &mut self.connections {
                if c.from > idx { c.from -= 1; }
                if c.to > idx { c.to -= 1; }
            }
            self.selected_agent = None;
            self.show_kb = false;
        }
    }
}
