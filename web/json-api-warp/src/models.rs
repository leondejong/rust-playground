use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Serialize, Clone)]
pub struct Item {
    id: u32,
    name: String,
    content: String,
    active: bool,
}

impl Item {
    fn new(id: u32, name: &str, content: &str, active: bool) -> Self {
        Item {
            id,
            active,
            name: name.to_owned(),
            content: content.to_owned(),
        }
    }
    fn update(&mut self, name: &str, content: &str, active: bool) {
        self.name = name.to_owned();
        self.content = content.to_owned();
        self.active = active;
    }
}

#[derive(Serialize)]
pub struct List {
    count: u32,
    items: Vec<Item>,
}

impl List {
    pub fn new() -> Self {
        Self {
            count: 0,
            items: vec![],
        }
    }
    pub fn all(&self) -> &Vec<Item> {
        &self.items
    }
    pub fn one(&self, id: u32) -> Option<&Item> {
        for item in self.items.iter() {
            if item.id == id {
                return Some(item);
            }
        }
        None
    }
    pub fn add(&mut self, name: &str, content: &str, active: bool) {
        self.count += 1;
        self.items
            .push(Item::new(self.count, name, content, active));
    }
    pub fn update(&mut self, id: u32, name: &str, content: &str, active: bool) {
        for item in self.items.iter_mut() {
            if item.id == id {
                item.update(name, content, active);
            }
        }
    }
    pub fn remove(&mut self, id: u32) {
        self.items.retain(|item| item.id != id);
    }
}

#[derive(Deserialize)]
pub struct Input {
    pub name: String,
    pub content: String,
    pub active: bool,
}

pub type State = Arc<Mutex<List>>;
