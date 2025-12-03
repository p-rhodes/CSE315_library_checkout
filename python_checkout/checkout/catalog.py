from .items import Item

class Catalog:
    def __init__(self):
        self.items=[]

    def add(self, item: Item):
        for i in range(len(self.items)):
            if self.items[i]._id.value == item._id.value:
                print(f"Catalog already has item {item.title}")
                raise ValueError

        self.items.append(item)

    def get(self, id: str):
        for i in range(len(self.items)):
            if self.items[i]._id.value == id:
                return self.items[i]

        return None
