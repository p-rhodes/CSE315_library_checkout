#Made by Parker Rhodes
#CSE315

class Member: #member class to act as a library user
              #stores a name string and a list of borrowed_ids()
    def __init__(self, name: str): #construct a new list and bind the name field when constructing
        self.name=name
        self.current_ids=[]

    def name(self) -> str: #getter for the name field
        return self.name
    
    def borrow(self, id): #Add an id to the current_ids list
        print(f"Borrowing <{id}> for User <{self.name}>") #console output

        if id in self.current_ids: #panic if duplicate borrow attempted
            print(f"User <{self.name}> already has item <{id}> borrowed!")
            raise ValueError

        if len(self.current_ids) > 4: #panic if max items borrowed
            print(f"User <{self.name}> has max items borrowed!")
            raise ValueError
        
        self.current_ids.append(id) #else, append id to the list

    def borrowed_ids(self): #getter for the current_ids list
        return self.current_ids

    def return_item(self, id): #remove an id from the current_ids list
        print(f"Returning <{id}> for User <{self.name}>") #console output

        if id in self.current_ids: #check if id is actually stored
            for i in range(len(self.current_ids)):
                if self.current_ids[i] == id:
                    del self.current_ids[i] #if stored, once found, delete and break
                    break

    def list_details(self, cat: Catalog): #return a list of nicely formatted strings listing a users borrowed items
                                          #must be passed a Catalog to fetch the details
        details=[] #list to output formatted strings

        for i in range(len(cat.items)): #for each item in the catalog
            if cat.items[i]._id.value in self.current_ids: #check if Catalog id is in users current_ids
               line: str=f"ID: {cat.items[i]._id.value} | Title: {cat.items[i].title} | Days Allowed: {cat.items[i].days_allowed()}"
               details.append(line) #if so, format a nice string with the details and add it to the output list

        return details #return the list of nicel formatted strings
