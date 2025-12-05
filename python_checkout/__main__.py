'''
Made by Parker Rhodes
CSE315
'''

#bring in classes from the checkout folder
from checkout.items import Id, Book, Dvd, Magazine
from checkout.catalog import Catalog
from checkout.member import Member


def demo() -> None:
    cat = Catalog()

    cat.add(Book(Id("B1"), "Rust for Humans"))
    cat.add(Book(Id("B2"), "Pythonic Patterns"))

    cat.add(Dvd(Id("D1"), "Taking Flight"))
    cat.add(Dvd(Id("D2"), "Patterns in Motion"))

    cat.add(Magazine(Id("M1"), "Cool Code Monthly"))
    cat.add(Magazine(Id("M2"), "Code Smells!"))

    m = Member("Clark")
    m.borrow("B1")
    m.borrow("D1")
    m.borrow("M2")
    print(m.name, "has borrowed:", m.borrowed_ids())

    print(f"\nPrinting details for user <{m.name}>...")
    for line in m.list_details(cat):
        print(f"  -> {line}")
    print()

    m.return_item("B1")
    print(m.name, "has borrowed:", m.borrowed_ids())

if __name__ == "__main__":
    demo()
