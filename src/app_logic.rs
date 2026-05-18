use eframe::egui;

#[derive(Clone)]
pub struct Item {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub created_at: String,
}

#[derive(PartialEq, Clone, Copy)]
pub enum ActiveField {
    Name,
    Description,
    None,
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
    pub active_field: ActiveField,  // Track field yang aktif
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
            active_field: ActiveField::None,
        }
    }

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
    
    pub fn delete_item(&mut self, index: usize) {
        if index < self.items.len() {
            self.items.remove(index);
            if let Some(selected) = self.selected_item {
                if selected == index {
                    self.selected_item = None;
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
    
    // Keyboard functions - now write to active field
    pub fn keyboard_add_char(&mut self, ch: char) {
        match self.active_field {
            ActiveField::Name => self.form_name.push(ch),
            ActiveField::Description => self.form_desc.push(ch),
            ActiveField::None => (),
        }
    }
    
    pub fn keyboard_backspace(&mut self) {
        match self.active_field {
            ActiveField::Name => { self.form_name.pop(); },
            ActiveField::Description => { self.form_desc.pop(); },
            ActiveField::None => (),
        }
    }
    
    pub fn keyboard_delete_word(&mut self) {
        match self.active_field {
            ActiveField::Name => {
                if let Some(last_space) = self.form_name.rfind(' ') {
                    self.form_name.truncate(last_space);
                } else {
                    self.form_name.clear();
                }
            },
            ActiveField::Description => {
                if let Some(last_space) = self.form_desc.rfind(' ') {
                    self.form_desc.truncate(last_space);
                } else {
                    self.form_desc.clear();
                }
            },
            ActiveField::None => (),
        }
    }
    
    pub fn keyboard_add_space(&mut self) {
        match self.active_field {
            ActiveField::Name => self.form_name.push(' '),
            ActiveField::Description => self.form_desc.push(' '),
            ActiveField::None => (),
        }
    }
    
    pub fn set_active_field(&mut self, field: ActiveField) {
        self.active_field = field;
        self.show_kb = true;
    }
}
