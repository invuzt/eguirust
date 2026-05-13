use eframe::egui;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub enum DataType {
    Number(f64),
    Text(String),
    Boolean(bool),
    Null,
}

impl Default for DataType {
    fn default() -> Self { DataType::Null }
}

#[derive(Clone, Debug)]
pub enum NodeType {
    Input,
    Process,
    Output,
    Function,
}

#[derive(Clone, Debug)]
pub struct Node {
    pub id: String,
    pub name: String,
    pub pos: egui::Pos2,
    pub color: egui::Color32,
    pub node_type: NodeType,
    pub inputs: HashMap<String, DataType>,
    pub outputs: HashMap<String, DataType>,
    pub config: HashMap<String, String>,
}

impl Node {
    pub fn new(id: String, name: String, pos: egui::Pos2, node_type: NodeType) -> Self {
        let (color, inputs, outputs) = match node_type {
            NodeType::Input => (
                egui::Color32::from_rgb(34, 197, 94),
                HashMap::new(),
                HashMap::from([("value".to_string(), DataType::Number(0.0))]),
            ),
            NodeType::Process => (
                egui::Color32::from_rgb(59, 130, 246),
                HashMap::from([("input".to_string(), DataType::Number(0.0))]),
                HashMap::from([("output".to_string(), DataType::Number(0.0))]),
            ),
            NodeType::Output => (
                egui::Color32::from_rgb(239, 68, 68),
                HashMap::from([("input".to_string(), DataType::Number(0.0))]),
                HashMap::new(),
            ),
            NodeType::Function => (
                egui::Color32::from_rgb(168, 85, 247),
                HashMap::new(),
                HashMap::from([("result".to_string(), DataType::Null)]),
            ),
        };
        Self { id, name, pos, color, node_type, inputs, outputs, config: HashMap::new() }
    }

    pub fn execute(&mut self, inputs: HashMap<String, DataType>) -> HashMap<String, DataType> {
        for (k, v) in inputs { self.inputs.insert(k, v); }
        match self.node_type {
            NodeType::Process => self.execute_process(),
            NodeType::Function => self.execute_function(),
            _ => self.outputs.clone(),
        }
    }

    fn execute_process(&mut self) -> HashMap<String, DataType> {
        let input = self.inputs.get("input").unwrap_or(&DataType::Number(0.0));
        let op = self.config.get("op").map(|s| s.as_str()).unwrap_or("add");
        let result = match (input, op) {
            (DataType::Number(a), "add") => DataType::Number(a + 1.0),
            (DataType::Number(a), "mul") => DataType::Number(a * 2.0),
            (DataType::Number(a), "div") => DataType::Number(a / 2.0),
            (DataType::Text(s), "upper") => DataType::Text(s.to_uppercase()),
            (DataType::Text(s), "lower") => DataType::Text(s.to_lowercase()),
            _ => DataType::Number(0.0),
        };
        self.outputs.insert("output".to_string(), result);
        self.outputs.clone()
    }

    fn execute_function(&mut self) -> HashMap<String, DataType> {
        let result = match self.config.get("func").map(|s| s.as_str()) {
            Some("random") => DataType::Number(rand::random::<f64>() * 100.0),
            Some("timestamp") => DataType::Number(std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as f64),
            _ => DataType::Null,
        };
        self.outputs.insert("result".to_string(), result);
        self.outputs.clone()
    }
}

#[derive(Clone, Debug)]
pub struct Connection {
    pub from_node: String,
    pub from_port: String,
    pub to_node: String,
    pub to_port: String,
}

pub struct AppState {
    pub nodes: Vec<Node>,
    pub connections: Vec<Connection>,
    pub selected_node: Option<usize>,
    pub is_running: bool,
    pub execution_log: Vec<String>,
    pub view_offset: egui::Vec2,
    pub zoom_factor: f32,
    pub temp_connection: Option<(String, String, egui::Pos2)>,
    next_id: usize,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            connections: Vec::new(),
            selected_node: None,
            is_running: false,
            execution_log: Vec::new(),
            view_offset: egui::vec2(0.0, 0.0),
            zoom_factor: 1.0,
            temp_connection: None,
            next_id: 0,
        }
    }

    fn gen_id(&mut self) -> String {
        let id = format!("node_{}", self.next_id);
        self.next_id += 1;
        id
    }

    pub fn add_node(&mut self, node_type: NodeType, pos: egui::Pos2) {
        let id = self.gen_id();
        let name = match node_type {
            NodeType::Input => format!("IN_{}", self.nodes.len()),
            NodeType::Process => format!("PRO_{}", self.nodes.len()),
            NodeType::Output => format!("OUT_{}", self.nodes.len()),
            NodeType::Function => format!("FN_{}", self.nodes.len()),
        };
        let name_clone = name.clone();
        self.nodes.push(Node::new(id, name, pos, node_type));
        self.execution_log.push(format!("Added: {}", name_clone));
    }

    pub fn delete_selected(&mut self) {
        if let Some(idx) = self.selected_node {
            let node_id = self.nodes[idx].id.clone();
            self.nodes.remove(idx);
            self.connections.retain(|c| c.from_node != node_id && c.to_node != node_id);
            self.selected_node = None;
            self.execution_log.push("Node deleted".to_string());
        }
    }

    pub fn add_connection(&mut self, from_node: String, from_port: String, to_node: String, to_port: String) {
        if from_node != to_node {
            self.connections.push(Connection { from_node, from_port, to_node, to_port });
            self.execution_log.push("Connection created".to_string());
        }
    }

    pub fn run_execution(&mut self) {
        self.execution_log.clear();
        let mut node_outputs: HashMap<String, HashMap<String, DataType>> = HashMap::new();
        
        for i in 0..self.nodes.len() {
            let node_id = self.nodes[i].id.clone();
            let mut input_data = HashMap::new();
            
            for conn in &self.connections {
                if conn.to_node == node_id {
                    if let Some(output_data) = node_outputs.get(&conn.from_node) {
                        if let Some(value) = output_data.get(&conn.from_port) {
                            input_data.insert(conn.to_port.clone(), value.clone());
                        }
                    }
                }
            }
            
            let outputs = self.nodes[i].execute(input_data);
            for (port, value) in &outputs {
                let value_str = match value {
                    DataType::Number(n) => format!("{}", n),
                    DataType::Text(s) => format!("\"{}\"", s),
                    DataType::Boolean(b) => format!("{}", b),
                    DataType::Null => "null".to_string(),
                };
                self.execution_log.push(format!("{} . {} = {}", self.nodes[i].name, port, value_str));
            }
            node_outputs.insert(node_id, outputs);
        }
    }
}
