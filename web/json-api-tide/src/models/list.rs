use super::item::Item;
use std::collections::BTreeMap;
use std::default::Default;

#[derive(Clone, Debug, PartialEq)]
pub struct List {
    count: u64,
    items: BTreeMap<u64, Item>,
}

impl List {
    pub fn new() -> Self {
        Self {
            count: 0,
            items: BTreeMap::new(),
        }
    }
    pub fn all(&self) -> Vec<&Item> {
        self.items.values().collect()
    }
    pub fn get(&self, id: u64) -> Option<&Item> {
        self.items.get(&id)
    }
    pub fn add(&mut self, name: &str, content: &str, active: bool) -> Option<&Item> {
        self.count += 1;
        let item = Item::new(self.count, name, content, active);
        self.items.insert(self.count, item);
        Some(&self.items[&self.count])
    }
    pub fn update(&mut self, id: u64, name: &str, content: &str, active: bool) -> Option<&Item> {
        if let Some(item) = self.items.get_mut(&id) {
            Some(item.update(name, content, active))
        } else {
            None
        }
    }
    pub fn remove(&mut self, id: u64) -> Option<Item> {
        self.items.remove(&id)
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
    fn new_list() {
        let list1 = List::new();
        let list2 = List {
            count: 0,
            items: BTreeMap::new(),
        };

        assert_eq!(list1, list2);
    }

    #[test]
    fn default_list() {
        let list1 = List::default();
        let list2 = List {
            count: 0,
            items: BTreeMap::new(),
        };

        assert_eq!(list1, list2);
    }

    #[test]
    fn get_item() {
        let mut map = BTreeMap::new();

        let item = Item::new(1, "name", "content", true);

        map.insert(1, item.clone());

        let list = List {
            count: 1,
            items: map,
        };

        assert_eq!(list.get(1), Some(&item));
    }

    #[test]
    fn all_items() {
        let mut map = BTreeMap::new();

        let item1 = Item::new(1, "name1", "content1", true);
        let item2 = Item::new(2, "name2", "content2", true);
        let item3 = Item::new(3, "name3", "content3", true);

        map.insert(1, item1.clone());
        map.insert(2, item2.clone());
        map.insert(3, item3.clone());

        let list = List {
            count: 3,
            items: map,
        };

        assert_eq!(list.all(), vec![&item1, &item2, &item3]);
    }

    #[test]
    fn add_item() {
        let mut map = BTreeMap::new();

        let item = Item::new(1, "name", "content", true);

        map.insert(1, item);

        let list1 = List {
            count: 1,
            items: map,
        };

        let mut list2 = List::new();

        list2.add("name", "content", true);

        assert_eq!(list1, list2);
    }

    #[test]
    fn update_item() {
        let mut map = BTreeMap::new();

        let item = Item::new(1, "name", "content", true);

        map.insert(1, item);

        let list1 = List {
            count: 1,
            items: map,
        };

        let mut list2 = List::new();

        list2.add("initial", "initial", false);
        list2.update(1, "name", "content", true);

        assert_eq!(list1, list2);
    }

    #[test]
    fn remove_item() {
        let map = BTreeMap::new();

        let list1 = List {
            count: 1,
            items: map,
        };

        let mut list2 = List::new();

        list2.add("name", "content", true);
        list2.remove(1);

        assert_eq!(list1, list2);
    }
}
