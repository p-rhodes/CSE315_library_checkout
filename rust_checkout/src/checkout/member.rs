//made by Parker Rhodes


pub struct Member {
    //Member struct that has a name string and a vector of strings that store ids from a catalog
    //struct
    name: String,
    current_ids: Vec<String>,
}

impl Member {
    pub fn new(name: String) -> Self {
        //Give a name to the constructor and assign it to a new Member
        //also construct a new vector
        Self {
            name: name,
            current_ids: vec![],
        }
    }

    pub fn name(&self) -> String {
        //Return a copy of the name string
        self.name.clone()
    }

    pub fn borrow(&mut self, id: String) {
        //Add an id from a catalog to the current_ids vector
        
        //console output
        println!("Borrowing <{}> for User <{}>", id, self.name);

        //check if member already has item borrowed
        if self.current_ids.contains(&id) {
            println!("User <{}> already has item <{}> borrowed!", self.name, id);
            return;
        }

        //check if member has hit the borrow limit
        if self.current_ids.len() >= 5 {
            println!("User <{}> has the maximum items borrowed!", self.name);
            return;
        }

        //else, add the id to the member's borrowed ids
        self.current_ids.push(id);
    }

    pub fn borrowed_ids(&self) -> Vec<String> {
        //Return a copy of the current_ids vector
        self.current_ids.clone()
    }

    pub fn return_item(&mut self, id: String) {
        //Remove an item from a user's current_ids vector

        //console output
        println!("Returning <{}> for User <{}>", id, self.name);

        //check if member actually has the id stored
        if self.current_ids.contains(&id) {
            //if yes, iterate through the current_ids vector
            //when the ids match, remove that id from the vector and break
            for i in 0..self.current_ids.len() {
                if self.current_ids[i] == id {
                    self.current_ids.remove(i);
                    break;
                }
            }
        }

        //if Member doesn't have the item borrowed, warn the user
        else {
            println!("Member <{}> does not have that Item borrowed!", self.name);
        }
    }
}

