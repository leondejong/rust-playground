use chrono::prelude::*;

cfg_if::cfg_if! {
    if #[cfg(test)] {
        fn now() -> DateTime<Utc> {
            Utc.ymd(1234, 5, 6).and_hms(7, 8, 9)
        }
    } else {
        fn now() -> DateTime<Utc> {
            Utc::now()
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Item {
    pub id: u64,
    pub active: bool,
    pub name: String,
    pub content: String,
    created: DateTime<Utc>,
    updated: DateTime<Utc>,
}

impl Item {
    pub fn new(id: u64, name: &str, content: &str, active: bool) -> Self {
        let datetime = now();
        Self {
            id,
            active,
            name: name.to_owned(),
            content: content.to_owned(),
            created: datetime,
            updated: datetime,
        }
    }
    pub fn update(&mut self, name: &str, content: &str, active: bool) -> &Self {
        self.name = name.to_owned();
        self.content = content.to_owned();
        self.active = active;
        self.updated = now();
        self
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_item() {
        let item1 = Item::new(0, "name", "content", true);
        let item2 = Item {
            id: 0,
            active: true,
            name: "name".to_owned(),
            content: "content".to_owned(),
            created: now(),
            updated: now(),
        };
        assert_eq!(&item1, &item2);
    }

    #[test]
    fn update_item() {
        let mut item1 = Item::new(0, "name1", "content1", false);
        let item2 = Item {
            id: 0,
            active: true,
            name: "name2".to_owned(),
            content: "content2".to_owned(),
            created: now(),
            updated: now(),
        };
        item1.update("name2", "content2", true);
        assert_eq!(item1, item2);
    }
}
