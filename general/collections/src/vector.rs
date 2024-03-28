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
}

#[derive(Debug, PartialEq)]
pub struct List {
    id: u32,
    items: Vec<Item>,
}

impl List {
    pub fn new() -> Self {
        Self {
            id: 0,
            items: vec![],
        }
    }
    pub fn all(&self) -> &Vec<Item> {
        &self.items
    }
    pub fn one(&self, id: u32) -> Option<&Item> {
        self.items.iter().find(|item| item.id == id)
    }
    pub fn add(&mut self, name: &str, content: &str, active: bool) {
        self.id += 1;
        self.items.push(Item::new(self.id, name, content, active))
    }
    pub fn update(&mut self, id: u32, name: &str, content: &str, active: bool) {
        for item in self.items.iter_mut() {
            if item.id == id {
                item.update(name, content, active);
            }
        }
    }
    pub fn remove(&mut self, id: u32) {
        self.items.retain(|item| item.id != id)
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
            items: vec![],
        };

        assert_eq!(reference, List::new());
    }

    #[test]
    fn list_one() {
        let item = Item {
            id: 1,
            active: true,
            name: "name".into(),
            content: "content".into(),
        };

        let list = List {
            id: 1,
            items: vec![item.clone()],
        };

        assert_eq!(Some(&item), list.one(1));
    }

    #[test]
    fn list_all() {
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
            content: "conten3".into(),
        };

        let list: List = List {
            id: 3,
            items: vec![item1.clone(), item2.clone(), item3.clone()],
        };

        assert_eq!(&vec![item1, item2, item3], list.all());
    }

    #[test]
    fn list_add() {
        let reference = List {
            id: 1,
            items: vec![Item {
                id: 1,
                active: true,
                name: "name".into(),
                content: "content".into(),
            }],
        };

        let mut list: List = List {
            id: 0,
            items: vec![],
        };

        list.add("name", "content", true);

        assert_eq!(reference, list);
    }

    #[test]
    fn list_update() {
        let reference = List {
            id: 1,
            items: vec![Item {
                id: 1,
                active: true,
                name: "name".into(),
                content: "content".into(),
            }],
        };

        let mut list = List {
            id: 1,
            items: vec![Item {
                id: 1,
                active: false,
                name: "n".into(),
                content: "c".into(),
            }],
        };

        list.update(1, "name", "content", true);

        assert_eq!(reference, list);
    }

    #[test]
    fn list_remove() {
        let reference = List {
            id: 1,
            items: vec![],
        };

        let mut list = List {
            id: 1,
            items: vec![Item {
                id: 1,
                active: true,
                name: "name".into(),
                content: "content".into(),
            }],
        };

        list.remove(1);

        assert_eq!(reference, list)
    }
}
