//Made by Parker Rhodes

pub trait Item { //Item trait. All items will have id, name, and days allowed. All of them must have these methods defined
    fn get_id(&self) -> String;
    fn get_name(&self) -> String;
    fn get_days(&self) -> u8;
}

pub struct Book {
    //Book struct that will inherit from Item
    id: String,
    name: String,
    max_days: u8
}

impl Book {
    //Constructor for the Book Item
    pub fn new(id: String, name: String) -> Self {
        Self {
            name: name,
            id: id,
            max_days: 7
        }
    }
}

impl Item for Book {
    //getters for the Book fields (Item trait)
    fn get_id(&self) -> String {
        self.id.clone()
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_days(&self) -> u8 {
        self.max_days
    }
}

pub struct Dvd {
    //Dvd struct that will inherit from item
    id: String,
    name: String,
    max_days: u8
}

impl Dvd {
    //Constructor for the Dvd Item
    pub fn new(id: String, name: String) -> Self {
        Self {
            name: name,
            id: id,
            max_days: 4
        }
    }
}

impl Item for Dvd {
    //Getters for the Dvd fields
    fn get_id(&self) -> String {
        self.id.clone()
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_days(&self) -> u8 {
        self.max_days
    }
}

pub struct Magazine {
    //Magazine struct that inherits from Item
    id: String,
    name: String,
    max_days: u8
}

impl Magazine {
    //Constructor for the Magazine
    pub fn new(id: String, name: String) -> Self {
        Self {
            name: name,
            id: id,
            max_days: 14
        }
    }
}

impl Item for Magazine {
    //Getters for the Magazine
    fn get_id(&self) -> String {
        self.id.clone()
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_days(&self) -> u8 {
        self.max_days
    }
}
