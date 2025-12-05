//made by Parker Rhodes

use crate::checkout::items::Item;

#[cfg(test)] //if testing, import items directly for setup_sample()
use crate::checkout::items::{Book, Dvd, Magazine};

pub struct Catalog {
    pub items: Vec<Box<dyn Item>>, //store items in the catalog
}

impl Catalog {
    pub fn new() -> Self {
        //construct a new vector to hold items and return
        Self {
            items: Vec::new(),
        }
    }

    pub fn add(&mut self, item: Box<dyn Item>) {
        //Add an item to the Catalog
        for i in 0..self.items.len() { //iterate through items vector
            if self.items[i].get_id() == item.get_id() { //check to ensure can't borrow duplicates
               return;
            }
        }

        if item.get_id().len() == 0 || item.get_name().len() == 0 { //check for bad strings in incoming item
            println!("Invalid name or id!");
            return;
        }

        self.items.push(item); //if item is good, add it
    }

    #[cfg(test)]
    pub fn get(&mut self, id: String) -> Box<dyn Item> {
        //if item is stored in the catalog, pop and remove it
        //if item is not stored in the catalog, return an empty object
        let empty_item: Box<dyn Item>=Box::new(Book::new(String::from(""), String::from("")));

        for i in 0..self.items.len() {
            if self.items[i].get_id() == id {
                return self.items.remove(i);
            }
        }

        return empty_item;
    }

    pub fn details_for(&self, borrowed_ids: &Vec::<String>) -> Vec<(String, String, u8)> {
        //needs to be passed a reference to the vector of borrowed_ids that a member struct stores.
        //Ex. cat.details_for(&member.borrowed_ids())

        //return a vector that stores tuples containing (id, title, max_days) for each item the
        //member that was provided has borrowed
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
        //add some items to a catalog for test purposes
        self.add(Box::new(Book::new(String::from("B1"), String::from("Rust for Humans"))));
        self.add(Box::new(Book::new(String::from("B2"), String::from("Pythonic Patterns"))));
        self.add(Box::new(Dvd::new(String::from("D1"), String::from("Taking Flight"))));
        self.add(Box::new(Magazine::new(String::from("M1"), String::from("Code Smells!"))));
    }
}
