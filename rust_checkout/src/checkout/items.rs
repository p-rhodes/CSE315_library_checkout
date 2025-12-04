pub trait Item {
    fn get_id(&self) -> String;
    fn get_name(&self) -> String;
    fn get_days(&self) -> u8;
}

pub struct Book {
    id: String,
    name: String,
    max_days: u8
}

impl Book {
    pub fn new(id: String, name: String) -> Self {
        Self {
            name: name,
            id: id,
            max_days: 7
        }
    }
}

impl Item for Book {
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
    id: String,
    name: String,
    max_days: u8
}

impl Dvd {
    pub fn new(id: String, name: String) -> Self {
        Self {
            name: name,
            id: id,
            max_days: 4
        }
    }
}

impl Item for Dvd {
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
