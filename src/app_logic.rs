use eframe::egui;

#[derive(Clone)]
pub struct Item {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub created_at: String,
}

pub struct AppState {
    pub items: Vec<Item>,
    pub next_id: u32,
    pub selected_item: Option<usize>,
    pub show_kb: bool,
    pub is_running: bool,
    pub view_offset: egui::Vec2,
    pub zoom_factor: f32,
    pub form_name: String,
    pub form_desc: String,
    pub edit_mode: bool,
    pub edit_id: u32,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            items: vec![
                Item {
                    id: 1,
                    name: "Sample Item 1".to_string(),
                    description: "This is a sample description".to_string(),
                    created_at: "2024-01-01".to_string(),
                },
                Item {
                    id: 2,
                    name: "Sample Item 2".to_string(),
                    description: "Another sample item".to_string(),
                    created_at: "2024-01-02".to_string(),
                },
            ],
            next_id: 3,
            selected_item: None,
            show_kb: false,
            is_running: false,
            view_offset: egui::vec2(0.0, 0.0),
            zoom_factor: 1.0,
            form_name: String::new(),
            form_desc: String::new(),
            edit_mode: false,
            edit_id: 0,
        }
    }

    // CREATE
    pub fn create_item(&mut self) {
        if !self.form_name.is_empty() {
            let now = chrono::Local::now().format("%Y-%m-%d").to_string();
            self.items.push(Item {
                id: self.next_id,
                name: self.form_name.clone(),
                description: self.form_desc.clone(),
                created_at: now,
            });
            self.next_id += 1;
            self.clear_form();
        }
    }

    // UPDATE
    pub fn start_edit(&mut self, index: usize) {
        if let Some(item) = self.items.get(index) {
            self.form_name = item.name.clone();
            self.form_desc = item.description.clone();
            self.edit_mode = true;
            self.edit_id = item.id;
            self.selected_item = Some(index);
        }
    }
    
    pub fn update_item(&mut self) {
        if let Some(index) = self.selected_item {
            if let Some(item) = self.items.get_mut(index) {
                if item.id == self.edit_id {
                    item.name = self.form_name.clone();
                    item.description = self.form_desc.clone();
                }
            }
        }
        self.clear_form();
        self.edit_mode = false;
        self.selected_item = None;
    }
    
    // DELETE
    pub fn delete_item(&mut self, index: usize) {
        if index < self.items.len() {
            self.items.remove(index);
            if let Some(selected) = self.selected_item {
                if selected == index {
                    self.selected_item = None;
                    self.show_kb = false;
                } else if selected > index {
                    self.selected_item = Some(selected - 1);
                }
            }
        }
    }
    
    pub fn clear_form(&mut self) {
        self.form_name.clear();
        self.form_desc.clear();
    }
    
    pub fn cancel_edit(&mut self) {
        self.clear_form();
        self.edit_mode = false;
        self.selected_item = None;
    }
}
