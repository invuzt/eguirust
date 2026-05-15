use eframe::egui;
use egui_graphs::{Graph, GraphView};
use petgraph::graph::NodeIndex;
use petgraph::prelude::*;
use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Debug)]
enum NodeType {
    Input,
    Process,
    Output,
}

pub struct GraphApp {
    graph: Graph<String, String>,
    selected_node: Option<NodeIndex>,
    node_type_map: HashMap<NodeIndex, NodeType>,
    counter: i32,
}

impl Default for GraphApp {
    fn default() -> Self {
        let mut graph = Graph::new();
        let mut node_type_map = HashMap::new();
        
        let node1 = graph.add_node("Start".to_string());
        let node2 = graph.add_node("Process".to_string());
        let node3 = graph.add_node("End".to_string());
        
        graph.add_edge(node1, node2, "flow".to_string());
        graph.add_edge(node2, node3, "result".to_string());
        
        node_type_map.insert(node1, NodeType::Input);
        node_type_map.insert(node2, NodeType::Process);
        node_type_map.insert(node3, NodeType::Output);
        
        Self {
            graph,
            selected_node: None,
            node_type_map,
            counter: 3,
        }
    }
}

impl eframe::App for GraphApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let screen_rect = ctx.input(|i| i.screen_rect());
        let screen_height = screen_rect.height();
        
        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            ui.add_space(screen_height * 0.05);
            ui.horizontal(|ui| {
                ui.add_space(12.0);
                
                if ui.button("➕ Add Node").clicked() {
                    let id = self.graph.add_node(format!("Node_{}", self.counter));
                    self.node_type_map.insert(id, NodeType::Process);
                    self.counter += 1;
                }
                
                if ui.button("🗑 Clear Selected").clicked() {
                    if let Some(node) = self.selected_node {
                        self.graph.remove_node(node);
                        self.node_type_map.remove(&node);
                        self.selected_node = None;
                    }
                }
                
                ui.add_space(12.0);
                if let Some(node) = self.selected_node {
                    if let Some(label) = self.graph.node_weight(node) {
                        ui.label(format!("Selected: {}", label));
                    }
                }
            });
            ui.add_space(8.0);
        });
        
        egui::TopBottomPanel::bottom("info").show(ctx, |ui| {
            ui.add_space(8.0);
            ui.horizontal(|ui| {
                ui.label(format!("Nodes: {} | Edges: {}", 
                    self.graph.node_count(), 
                    self.graph.edge_count()
                ));
                ui.add_space(20.0);
                ui.label("💡 Tip: Click node to select, drag to move, pinch to zoom");
            });
            ui.add_space(8.0);
        });
        
        egui::CentralPanel::default().show(ctx, |ui| {
            let mut graph_view = GraphView::new(&mut self.graph);
            let response = ui.add(&mut graph_view);
            
            if let Some(selected) = graph_view.selected_nodes().iter().next() {
                self.selected_node = Some(*selected);
            }
            
            if response.secondary_clicked() {
                let mut menu = egui::popup::popup_menu(ui, response.rect, "node_menu");
                if menu.show().clicked() {
                    // Handle menu items
                }
            }
        });
    }
}
