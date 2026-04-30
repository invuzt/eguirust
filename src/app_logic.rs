use std::collections::HashMap;
use eframe::egui;

#[allow(dead_code)]
pub struct Node {
    pub pos: egui::Pos2,
    pub is_server: bool,
}

pub struct AppState {
    pub nodes: HashMap<String, Node>,
    pub app_name: String,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            app_name: "VUZT CORE".to_string(),
        }
    }
    pub fn add_node(&mut self) {
        let id = self.nodes.len();
        self.nodes.insert(format!("N-{}", id), Node {
            pos: egui::pos2(150.0, 150.0),
            is_server: id == 0,
        });
    }
}
