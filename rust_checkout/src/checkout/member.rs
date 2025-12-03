//made by Parker Rhodes


pub struct Member {
    name: String,
    current_ids: Vec<String>,
}

impl Member {
    pub fn new(name: String) -> Self {
        Self {
            name: name,
            current_ids: vec![],
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn borrow(&mut self, id: String) {
        println!("Borrowing <{}> for User <{}>", id, self.name);

        if self.current_ids.contains(&id) {
            println!("User <{}> already has item <{}> borrowed!", self.name, id);
            return;
        }

        self.current_ids.push(id);
    }

    pub fn borrowed_ids(&self) -> Vec<String> {
        self.current_ids.clone()
    }

    pub fn return_item(&mut self, id: String) {
        println!("Returning <{}> for User <{}>", id, self.name);

        if self.current_ids.contains(&id) {
            for i in 0..self.current_ids.len() {
                if self.current_ids[i] == id {
                    self.current_ids.remove(i);
                    break;
                }
            }
        }

        else {
            println!("Member <{}> does not have that Item borrowed!", self.name);
        }
    }
}

