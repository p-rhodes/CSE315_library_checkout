'''
Made by Parker Rhodes
CSE315
'''

#bring in classes from the checkout folder
from checkout.items import Id, Book, Dvd, Magazine
from checkout.catalog import Catalog
from checkout.member import Member


def demo() -> None:
    cat = Catalog() #Construct a new catalog

    #add a bunch of items to the catalog
    cat.add(Book(Id("B1"), "Rust for Humans"))
    cat.add(Book(Id("B2"), "Pythonic Patterns"))
    cat.add(Dvd(Id("D1"), "Taking Flight"))
    cat.add(Dvd(Id("D2"), "Patterns in Motion"))
    cat.add(Magazine(Id("M1"), "Cool Code Monthly"))
    cat.add(Magazine(Id("M2"), "Code Smells!"))

    #make a new member and borrow some items for them
    m = Member("Clark")
    m.borrow("B1")
    m.borrow("D1")
    m.borrow("M2")
    print(m.name, "has borrowed:", m.borrowed_ids()) #print details for Clark

    print(f"\nPrinting details for user <{m.name}>...") #print details for Clark's borrowed items
                                                        #in nice format
    for line in m.list_details(cat):
        print(f"  -> {line}")
    print()

    m.return_item("B1") #return an item to show functionality
    print(m.name, "has borrowed:", m.borrowed_ids())

if __name__ == "__main__": #only run demo() function if this script is inside __main__.py file
    demo()
