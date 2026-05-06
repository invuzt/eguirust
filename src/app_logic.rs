use eframe::egui;

pub struct Node {
    pub label: String,
    pub pos: egui::Pos2,
}

pub struct AppState {
    pub app_name: String,
    pub selected_node_idx: Option<usize>,
    pub nodes: Vec<Node>,
    pub show_kb: bool,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            app_name: "VUZT NATIVE".to_string(),
            selected_node_idx: None,
            nodes: Vec::new(),
            show_kb: false,
        }
    }

    pub fn add_node(&mut self) {
        let id = self.nodes.len();
        // Logika posisi: menyebar ke samping lalu ke bawah (grid-like)
        let x = 30.0 + ((id % 3) as f32 * 100.0);
        let y = 120.0 + ((id / 3) as f32 * 60.0);
        
        self.nodes.push(Node {
            label: format!("ID:{}", id),
            pos: egui::pos2(x, y),
        });
    }
}
