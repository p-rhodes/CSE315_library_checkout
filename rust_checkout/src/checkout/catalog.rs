//made by Parker Rhodes

use crate::checkout::items::Item;

pub struct Catalog {
    items: Vec<Box<dyn Item>>,
}

impl Catalog {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
        }
    }

    pub fn add(&mut self, item: Box<dyn Item>) {
        self.items.push(item);
    }

    pub fn details_for(&self, borrowed_ids: &Vec::<String>) -> Vec<(String, String, u8)> {
        let mut details: Vec<(String, String, u8)>=vec![];

        for i in 0..self.items.len() {
            if borrowed_ids.contains(&self.items[i].get_id()) {
                details.push((self.items[i].get_id(), self.items[i].get_name(), self.items[i].get_days()));
            }
        }

        details
    }
}
