from .items import Item #import the item class since we will add them to the catalog

class Catalog:
    def __init__(self): #construct a new list when constructing new catalog
        self.items=[]

    def add(self, item: Item): #Take an item and add it to the Catalog
        for i in range(len(self.items)):
            if self.items[i]._id.value == item._id.value: #check for duplicate borrows!
                print(f"Catalog already has item {item.title}")
                raise ValueError #Panic if Catalog already has that item

        self.items.append(item) #otherwise, add it to the list

    def get(self, id: str): #check if catalog contains item and return it if so
        for i in range(len(self.items)):
            if self.items[i]._id.value == id:
                return self.items[i]

        return None #if it doesn't contain that item, return None
