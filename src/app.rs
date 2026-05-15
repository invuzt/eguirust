use eframe::egui;
use egui_graphs::{
    DefaultEdgeShape, DefaultNodeShape, Graph, GraphView, Settings, Interaction,
};
use petgraph::graph::NodeIndex;
use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq)]
enum NodeType {
    Input,
    Process,
    Output,
}

pub struct GraphApp {
    graph: Graph<String, String>,
    selected_node: Option<NodeIndex>,
    node_data: HashMap<NodeIndex, NodeType>,
    counter: i32,
}

impl Default for GraphApp {
    fn default() -> Self {
        let mut graph = Graph::new();
        let node_data = HashMap::new();
        
        // Create example nodes
        let node1 = graph.add_node("Start".to_string());
        let node2 = graph.add_node("Process".to_string());
        let node3 = graph.add_node("End".to_string());
        
        graph.add_edge(node1, node2, "flow".to_string());
        graph.add_edge(node2, node3, "result".to_string());
        
        let mut node_data = HashMap::new();
        node_data.insert(node1, NodeType::Input);
        node_data.insert(node2, NodeType::Process);
        node_data.insert(node3, NodeType::Output);
        
        Self {
            graph,
            selected_node: None,
            node_data,
            counter: 0,
        }
    }
}

impl eframe::App for GraphApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let screen_rect = ctx.input(|i| i.screen_rect());
        let screen_height = screen_rect.height();
        
        // Top panel with controls
        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            ui.add_space(screen_height * 0.05);
            ui.horizontal(|ui| {
                ui.add_space(12.0);
                
                if ui.button("➕ Add Node").clicked() {
                    let id = self.graph.add_node(format!("Node_{}", self.counter));
                    self.node_data.insert(id, NodeType::Process);
                    self.counter += 1;
                }
                
                if ui.button("🗑 Clear Selected").clicked() {
                    if let Some(node) = self.selected_node {
                        self.graph.remove_node(node);
                        self.node_data.remove(&node);
                        self.selected_node = None;
                    }
                }
                
                if ui.button("🔄 Force Layout").clicked() {
                    self.graph = self.graph.clone();
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
        
        // Bottom panel with info
        egui::TopBottomPanel::bottom("info").show(ctx, |ui| {
            ui.add_space(8.0);
            ui.horizontal(|ui| {
                ui.label(format!("Nodes: {} | Edges: {}", 
                    self.graph.node_count(), 
                    self.graph.edge_count()
                ));
                ui.add_space(20.0);
                ui.label("💡 Tip: Click node to select, drag to move, zoom with pinch");
            });
            ui.add_space(8.0);
        });
        
        // Central panel with graph
        egui::CentralPanel::default().show(ctx, |ui| {
            // Graph settings
            let settings = Settings {
                enable_zoom: true,
                enable_pan: true,
                enable_drag: true,
                enable_selection: true,
                selection_color: egui::Color32::from_rgb(100, 200, 255),
                selection_thickness: 3.0,
                zoom_speed: 0.1,
                min_zoom: 0.2,
                max_zoom: 3.0,
                edge_radius: 10.0,
                ..Default::default()
            };
            
            let interaction = Interaction {
                drag_node: true,
                drag_view: true,
                select_nodes: true,
                ..Default::default()
            };
            
            // Create graph view
            let mut graph_view = GraphView::new(&mut self.graph)
                .with_settings(settings)
                .with_interaction(interaction)
                .with_node_shape(Box::new(|_, _, node_label, _| {
                    // Custom node shape based on type
                    let color = match self.node_data.get(&node_label) {
                        Some(NodeType::Input) => egui::Color32::from_rgb(34, 197, 94),
                        Some(NodeType::Process) => egui::Color32::from_rgb(59, 130, 246),
                        Some(NodeType::Output) => egui::Color32::from_rgb(239, 68, 68),
                        None => egui::Color32::from_rgb(168, 85, 247),
                    };
                    DefaultNodeShape::new(color)
                }))
                .with_edge_shape(Box::new(|_, _, _| {
                    DefaultEdgeShape::new(egui::Color32::from_rgb(238, 207, 60))
                }));
            
            let response = ui.add(&mut graph_view);
            
            // Handle node selection
            if let Some(selected) = graph_view.selected_nodes().iter().next() {
                self.selected_node = Some(*selected);
            }
            
            // Show context menu on right click
            if response.context_clicked() {
                let mut menu = egui::popup::PopupMenu::new("node_menu", response.id);
                menu.show(ui, |ui| {
                    if ui.button("Add Input Node").clicked() {
                        let id = self.graph.add_node("Input".to_string());
                        self.node_data.insert(id, NodeType::Input);
                        self.counter += 1;
                        ui.close_menu();
                    }
                    if ui.button("Add Process Node").clicked() {
                        let id = self.graph.add_node("Process".to_string());
                        self.node_data.insert(id, NodeType::Process);
                        self.counter += 1;
                        ui.close_menu();
                    }
                    if ui.button("Add Output Node").clicked() {
                        let id = self.graph.add_node("Output".to_string());
                        self.node_data.insert(id, NodeType::Output);
                        self.counter += 1;
                        ui.close_menu();
                    }
                });
            }
        });
    }
}
