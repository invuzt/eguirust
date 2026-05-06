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
        // Spawn dengan sedikit offset agar tidak menumpuk tepat di satu titik
        let x = 50.0 + (id as f32 * 20.0 % 200.0);
        let y = 150.0 + (id as f32 * 30.0 % 300.0);
        
        self.nodes.push(Node {
            label: format!("NODE {}", id),
            pos: egui::pos2(x, y),
        });
    }
}
