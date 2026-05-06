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
    pub link_source: Option<usize>,
    pub show_kb: bool,
    pub is_running: bool,
    // Navigasi Canvas
    pub view_offset: egui::Vec2,
    pub zoom_factor: f32,
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
            view_offset: egui::vec2(0.0, 0.0),
            zoom_factor: 1.0,
        }
    }

    pub fn add_agent(&mut self) {
        let id = self.agents.len();
        // Spawn relatif terhadap view agar muncul di tengah layar yang terlihat
        let spawn_pos = egui::pos2(150.0, 300.0);
        self.agents.push(Agent {
            name: format!("AGENT_{}", id),
            pos: spawn_pos,
            color: egui::Color32::from_rgb(0, 120, 255),
        });
    }

    pub fn spawn_child(&mut self, parent_idx: usize) {
        let parent_pos = self.agents[parent_idx].pos;
        let id = self.agents.len();
        let new_pos = parent_pos + egui::vec2(150.0, 0.0);
        self.agents.push(Agent {
            name: format!("AGENT_{}", id),
            pos: new_pos,
            color: egui::Color32::from_rgb(0, 200, 150),
        });
        self.connections.push(Connection { from: parent_idx, to: id, message: "Sync".to_string() });
    }

    pub fn delete_selected(&mut self) {
        if let Some(idx) = self.selected_agent {
            self.agents.remove(idx);
            self.connections.retain(|c| c.from != idx && c.to != idx);
            for c in &mut self.connections {
                if c.from > idx { c.from -= 1; }
                if c.to > idx { c.to -= 1; }
            }
            self.selected_agent = None;
            self.show_kb = false;
        }
    }
}
