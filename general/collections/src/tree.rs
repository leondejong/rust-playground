use chrono::prelude::*;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::collections::BTreeMap;
use std::default::Default;

cfg_if::cfg_if! {
    if #[cfg(test)] {
        fn now() -> DateTime<Utc> {
            Utc.with_ymd_and_hms(1234, 5, 6, 7, 8, 9).unwrap()
        }
    } else {
        fn now() -> DateTime<Utc> {
            Utc::now()
        }
    }
}

fn random(length: usize) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect::<String>()
}

#[derive(Clone, Debug, PartialEq)]
pub struct Item {
    id: String,
    active: bool,
    name: String,
    content: String,
    created: DateTime<Utc>,
    updated: DateTime<Utc>,
}

impl Item {
    pub fn new(id: &str, name: &str, content: &str, active: bool) -> Self {
        let datetime = now();
        Self {
            active,
            id: id.into(),
            name: name.into(),
            content: content.into(),
            created: datetime,
            updated: datetime,
        }
    }
    pub fn update(&mut self, name: &str, content: &str, active: bool) -> &Self {
        self.active = active;
        self.name = name.into();
        self.content = content.into();
        self.updated = now();
        self
    }
    pub fn id(&self) -> &str {
        &self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn content(&self) -> &str {
        &self.content
    }
    pub fn active(&self) -> bool {
        self.active
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct List {
    items: BTreeMap<String, Item>,
}

impl List {
    pub fn new() -> Self {
        Self {
            items: BTreeMap::new(),
        }
    }
    pub fn all(&self) -> Vec<&Item> {
        self.items.values().collect()
    }
    pub fn one(&self, id: &str) -> Option<&Item> {
        self.items.get(id)
    }
    pub fn add(&mut self, name: &str, content: &str, active: bool) -> Option<&Item> {
        let id = random(16);
        let item = Item::new(&id, name, content, active);
        self.items.insert(id.clone(), item);
        Some(&self.items[&id])
    }
    pub fn update(&mut self, id: &str, name: &str, content: &str, active: bool) -> Option<&Item> {
        if let Some(item) = self.items.get_mut(id) {
            Some(item.update(name, content, active))
        } else {
            None
        }
    }
    pub fn remove(&mut self, id: &str) -> Option<Item> {
        self.items.remove(id)
    }
}

impl Default for List {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn item_new() {
        let datetime = now();

        let reference = Item {
            id: "id".into(),
            name: "name".into(),
            content: "content".into(),
            created: datetime,
            updated: datetime,
            active: true,
        };

        let item = Item::new("id", "name", "content", true);

        assert_eq!(reference, item);
    }

    #[test]
    fn item_update() {
        let datetime = now();

        let reference = Item {
            id: "id".into(),
            name: "name".into(),
            content: "content".into(),
            created: datetime,
            updated: datetime,
            active: true,
        };

        let mut item = Item {
            id: "id".into(),
            name: "n".into(),
            content: "c".into(),
            created: datetime,
            updated: datetime,
            active: true,
        };

        item.update("name", "content", true);

        assert_eq!(reference, item);
    }

    #[test]
    fn item_get() {
        let datetime = now();

        let item = Item {
            id: "id".into(),
            name: "name".into(),
            content: "content".into(),
            created: datetime,
            updated: datetime,
            active: true,
        };

        assert_eq!(item.id(), "id");
        assert_eq!(item.name(), "name");
        assert_eq!(item.content(), "content");
        assert_eq!(item.active(), true);
    }

    #[test]
    fn list_new() {
        let reference = List {
            items: BTreeMap::new(),
        };

        let list = List::new();

        assert_eq!(reference, list);
    }

    #[test]
    fn list_default() {
        let reference = List {
            items: BTreeMap::new(),
        };

        let list = List::default();

        assert_eq!(reference, list);
    }

    #[test]
    fn list_get() {
        let datetime = now();

        let mut items = BTreeMap::new();

        let item = Item {
            id: "id".into(),
            name: "name".into(),
            content: "content".into(),
            created: datetime,
            updated: datetime,
            active: true,
        };

        items.insert("id".into(), item.clone());

        let list = List { items };

        assert_eq!(list.one("id"), Some(&item));
    }

    #[test]
    fn list_all() {
        let datetime = now();

        let mut items = BTreeMap::new();

        let item1 = Item {
            id: "id1".into(),
            name: "name1".into(),
            content: "content1".into(),
            created: datetime,
            updated: datetime,
            active: true,
        };

        let item2 = Item {
            id: "id2".into(),
            name: "name2".into(),
            content: "content2".into(),
            created: datetime,
            updated: datetime,
            active: true,
        };

        let item3 = Item {
            id: "id3".into(),
            name: "name3".into(),
            content: "content3".into(),
            created: datetime,
            updated: datetime,
            active: true,
        };

        items.insert("id1".into(), item1.clone());
        items.insert("id2".into(), item2.clone());
        items.insert("id3".into(), item3.clone());

        let list = List { items };

        assert_eq!(list.all(), vec![&item1, &item2, &item3]);
    }

    #[test]
    fn list_add() {
        let datetime = now();

        let mut list = List {
            items: BTreeMap::new(),
        };

        let item = list.add("name", "content", true).unwrap().clone();

        let mut items = BTreeMap::new();

        let item = Item {
            id: item.id().into(),
            name: "name".into(),
            content: "content".into(),
            created: datetime,
            updated: datetime,
            active: true,
        };

        items.insert(item.id().into(), item);

        let reference = List { items };

        assert_eq!(reference, list);
    }

    #[test]
    fn list_update() {
        let datetime = now();

        let mut reference_items = BTreeMap::new();
        let mut items = BTreeMap::new();

        let reference_item = Item {
            id: "id".into(),
            name: "name".into(),
            content: "content".into(),
            created: datetime,
            updated: datetime,
            active: true,
        };

        let item = Item {
            id: "id".into(),
            name: "n".into(),
            content: "c".into(),
            created: datetime,
            updated: datetime,
            active: false,
        };

        reference_items.insert("id".into(), reference_item);
        items.insert("id".into(), item);

        let reference_list = List {
            items: reference_items,
        };

        let mut list = List { items };

        list.update("id".into(), "name", "content", true);

        assert_eq!(reference_list, list);
    }

    #[test]
    fn list_remove() {
        let datetime = now();

        let mut items = BTreeMap::new();

        let item = Item {
            id: "id".into(),
            name: "name".into(),
            content: "content".into(),
            created: datetime,
            updated: datetime,
            active: true,
        };

        items.insert("id".into(), item);

        let reference = List {
            items: BTreeMap::new(),
        };

        let mut list = List { items };

        list.remove("id".into());

        assert_eq!(reference, list);
    }
}
