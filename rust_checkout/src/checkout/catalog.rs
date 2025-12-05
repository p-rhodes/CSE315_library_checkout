//made by Parker Rhodes

use crate::checkout::items::Item;

#[cfg(test)]
use crate::checkout::items::Book;
#[cfg(test)]
use crate::checkout::items::Dvd;

pub struct Catalog {
    pub items: Vec<Box<dyn Item>>,
}

impl Catalog {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
        }
    }

    pub fn add(&mut self, item: Box<dyn Item>) {
        for i in 0..self.items.len() {
            if self.items[i].get_id() == item.get_id() {
               return;
            }
        }

        if item.get_id().len() == 0 || item.get_name().len() == 0 {
            println!("Invalid name or id!");
            return;
        }

        self.items.push(item);
    }

    #[cfg(test)]
    pub fn get(&mut self, id: String) -> Box<dyn Item> {
        let empty_item: Box<dyn Item>=Box::new(Book::new(String::from(""), String::from("")));

        for i in 0..self.items.len() {
            if self.items[i].get_id() == id {
                return self.items.remove(i);
            }
        }

        return empty_item;
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

    #[cfg(test)]
    pub fn setup_sample(&mut self) {
        self.add(Box::new(Book::new(String::from("B1"), String::from("Rust for Humans"))));
        self.add(Box::new(Book::new(String::from("B2"), String::from("Pythonic Patterns"))));
        self.add(Box::new(Dvd::new(String::from("D1"), String::from("Taking Flight"))));
    }
}
