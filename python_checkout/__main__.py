from checkout.items import Id, Book, Dvd
from checkout.catalog import Catalog
from checkout.member import Member

def demo() -> None:
    cat = Catalog()

    cat.add(Book(Id("B1"), "Rust for Humans"))
    cat.add(Book(Id("B2"), "Pythonic Patterns"))
    cat.add(Dvd(Id("D1"), "Taking Flight"))
    cat.add(Dvd(Id("D2"), "Patterns in Motion"))

    m = Member("Clark")
    m.borrow("B1")
    m.borrow("D1")
    print(m.name, "has borrowed:", m.borrowed_ids())

    for line in m.list_details(cat):
        print(f"  -> {line}")

    m.return_item("B1")
    print(m.name, "has borrowed:", m.borrowed_ids())


    print(f"\n{type(cat.items[0])}")
    print(f"{m.borrowed_ids}")

if __name__ == "__main__":
    demo()
