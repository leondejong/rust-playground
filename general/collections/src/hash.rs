use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
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
        self.active = active;
        self.name = name.into();
        self.content = content.into();
        self
    }
    fn id(&self) -> u32 {
        self.id
    }
}

#[derive(Debug, PartialEq)]
pub struct List {
    id: u32,
    items: HashMap<u32, Item>,
}

impl List {
    pub fn new() -> Self {
        Self {
            id: 0,
            items: HashMap::new(),
        }
    }
    pub fn all(&self) -> Vec<&Item> {
        let mut list = self.items.values().collect::<Vec<&Item>>();
        list.sort_by_key(|v| v.id());
        list
    }
    pub fn one(&self, id: u32) -> Option<&Item> {
        self.items.get(&id)
    }
    pub fn add(&mut self, name: &str, content: &str, active: bool) -> Option<&Item> {
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn item_new() {
        // benchmark
        let reference = Item {
            id: 1,
            active: true,
            name: "name".into(),
            content: "content".into(),
        };

        let item = Item::new(1, "name", "content", true);

        assert_eq!(reference, item);
    }

    #[test]
    fn item_update() {
        let reference = Item {
            id: 1,
            active: true,
            name: "name".into(),
            content: "content".into(),
        };

        let mut item = Item {
            id: 1,
            active: false,
            name: "n".into(),
            content: "c".into(),
        };

        item.update("name", "content", true);

        assert_eq!(reference, item);
    }

    #[test]
    fn list_new() {
        let reference = List {
            id: 0,
            items: HashMap::new(),
        };

        assert_eq!(reference, List::new());
    }

    #[test]
    fn list_one() {
        let mut items = HashMap::new();

        let item = Item {
            id: 1,
            active: true,
            name: "name".into(),
            content: "content".into(),
        };

        items.insert(1, item.clone());

        let list = List { id: 1, items };

        assert_eq!(Some(&item), list.one(1));
    }

    #[test]
    fn list_all() {
        let mut items = HashMap::new();

        let item1 = Item {
            id: 1,
            active: true,
            name: "name1".into(),
            content: "content1".into(),
        };

        let item2 = Item {
            id: 2,
            active: true,
            name: "name2".into(),
            content: "content2".into(),
        };

        let item3 = Item {
            id: 3,
            active: true,
            name: "name3".into(),
            content: "content3".into(),
        };

        items.insert(1, item1.clone());
        items.insert(2, item2.clone());
        items.insert(3, item3.clone());

        let list: List = List { id: 3, items };

        assert_eq!(vec![&item1, &item2, &item3], list.all());
    }

    #[test]
    fn list_add() {
        let mut items = HashMap::new();

        items.insert(
            1,
            Item {
                id: 1,
                active: true,
                name: "name".into(),
                content: "content".into(),
            },
        );

        let reference = List { id: 1, items };

        let mut list: List = List {
            id: 0,
            items: HashMap::new(),
        };

        list.add("name", "content", true);

        assert_eq!(reference, list);
    }

    #[test]
    fn list_update() {
        let mut reference_items = HashMap::new();
        let mut items = HashMap::new();

        reference_items.insert(
            1,
            Item {
                id: 1,
                active: true,
                name: "name".into(),
                content: "content".into(),
            },
        );

        items.insert(
            1,
            Item {
                id: 1,
                active: true,
                name: "name".into(),
                content: "content".into(),
            },
        );

        let reference = List {
            id: 1,
            items: reference_items,
        };

        let mut list = List { id: 1, items };

        list.update(1, "name", "content", true);

        assert_eq!(reference, list);
    }

    #[test]
    fn list_remove() {
        let mut items = HashMap::new();

        items.insert(
            1,
            Item {
                id: 1,
                active: true,
                name: "name".into(),
                content: "content".into(),
            },
        );

        let reference = List {
            id: 1,
            items: HashMap::new(),
        };

        let mut list = List { id: 1, items };

        list.remove(1);

        assert_eq!(reference, list)
    }
}
