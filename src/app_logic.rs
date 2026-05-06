use eframe::egui;

pub struct Node {
    pub pos: egui::Pos2,
    pub is_server: bool,
}

pub struct AppState {
    pub app_name: String,
    pub show_kb: bool,
    pub nodes: Vec<Node>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            app_name: "VUZT".to_string(),
            show_kb: false,
            nodes: Vec::new(),
        }
    }

    pub fn add_node(&mut self) {
        self.nodes.push(Node {
            pos: egui::pos2(10.0, 10.0),
            is_server: false,
        });
    }
}
