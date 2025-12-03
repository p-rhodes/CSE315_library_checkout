class Member:
    def __init__(self, name: str):
        self.name=name
        self.current_ids=[]

    def name(self) -> str:
        return self.name
    
    def borrow(self, id):
        print(f"Borrowing <{id}> for User <{self.name}>")

        if id in self.current_ids:
            print(f"User <{self.name}> already has item <{id}> borrowed!")
            raise ValueError

        if len(self.current_ids) > 4:
            print(f"User <{self.name}> alraedy has item<{id}> borrowed!")
            raise ValueError
        
        self.current_ids.append(id)

    def borrowed_ids(self):
        return self.current_ids

    def return_item(self, id):
        print(f"Returning <{id}> for User <{self.name}>")

        if id in self.current_ids:
            for i in range(len(self.current_ids)):
                if self.current_ids[i] == id:
                    del self.current_ids[i]
                    break

    def list_details(self, cat: Catalog):
        details=[] #list to output formatted strings

        for i in range(len(cat.items)):
            if cat.items[i]._id.value in self.current_ids:
               line: str=f"ID: {cat.items[i]._id.value} | Title: {cat.items[i].title} | Days Allowed: {cat.items[i].days_allowed()}"
               details.append(line)

        return details
