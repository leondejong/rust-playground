use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Serialize, Clone)]
pub struct Item {
    id: u32,
    active: bool,
    name: String,
    content: String,
}

impl Item {
    fn new(id: u32, name: &str, content: &str, active: bool) -> Self {
        Item {
            id,
            active,
            name: name.into(),
            content: content.into(),
        }
    }
    fn update(&mut self, name: &str, content: &str, active: bool) -> &Self {
        self.name = name.into();
        self.content = content.into();
        self.active = active;
        self
    }
}

#[derive(Serialize)]
pub struct List {
    id: u32,
    items: BTreeMap<u32, Item>,
}

impl List {
    pub fn new() -> Self {
        Self {
            id: 0,
            items: BTreeMap::new(),
        }
    }
    pub fn index(&self) -> Vec<&Item> {
        self.items.values().collect()
    }
    pub fn get(&self, id: u32) -> Option<&Item> {
        self.items.get(&id)
    }
    pub fn insert(&mut self, name: &str, content: &str, active: bool) -> Option<&Item> {
        self.id += 1;
        let item = Item::new(self.id, name, content, active);
        self.items.insert(self.id, item);
        Some(&self.items[&self.id])
    }
    pub fn update(&mut self, id: u32, name: &str, content: &str, active: bool) -> Option<&Item> {
        if let Some(item) = self.items.get_mut(&id) {
            Some(item.update(name, content, active))
        } else {
            None
        }
    }
    pub fn remove(&mut self, id: u32) -> Option<Item> {
        self.items.remove(&id)
    }
}

#[derive(Deserialize)]
pub struct Input {
    pub name: String,
    pub content: String,
    pub active: bool,
}

pub type State = Arc<Mutex<List>>;
