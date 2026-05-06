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
            app_name: "VUZT IDE".to_string(),
            selected_node_idx: None,
            nodes: Vec::new(),
            show_kb: false,
        }
    }

    pub fn add_node(&mut self) {
        let id = self.nodes.len();
        let new_node = Node {
            label: format!("Node_{}", id),
            // Simulasi posisi acak agar tidak tumpang tindih
            pos: egui::pos2(50.0 + (id as f32 * 10.0), 100.0 + (id as f32 * 20.0)),
        };
        self.nodes.push(new_node);
    }
}
