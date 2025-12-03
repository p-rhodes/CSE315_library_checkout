import pytest

from checkout.catalog import Catalog
from checkout.items import Book, Dvd, Id
from checkout.member import Member

def setup_sample():
    cat = Catalog()
    cat.add(Book(Id("B1"), "Rust for Humans"))
    cat.add(Book(Id("B2"), "Pythonic Patterns"))
    cat.add(Dvd(Id("D1"), "Taking Flight"))
    return cat

def test_add_and_get():
    cat = setup_sample()
    assert cat.get("B1").title == "Rust for Humans"
    assert cat.get("D1").days_allowed() == 7

def test_duplicate_id_rejected():
    cat = setup_sample()
    with pytest.raises(ValueError):
        cat.add(Book(Id("B1"), "Duplicate"))

def test_member_borrow_and_return():
    cat = setup_sample()
    m = Member("Clark")
    m.borrow("B1")
    assert m.borrowed_ids() == ["B1"]
    m.return_item("B1")
    assert m.borrowed_ids() == []

def test_member_cannot_borrow_twice():
    m = Member("X")
    m.borrow("K1")
    with pytest.raises(ValueError):
        m.borrow("K1")

def test_member_limit_enforced():
    m = Member("X")
    for i in range(5):
        m.borrow(f"K{i}")
    with pytest.raises(ValueError):
        m.borrow("K5")

def test_details_lines():
    cat = setup_sample()
    m = Member("Y")
    m.borrow("B2")
    lines = m.list_details(cat)
    assert len(lines) == 1
    assert "Pythonic Patterns" in lines[0]
    assert "Days Allowed: 14" in lines[0]

def test_type_catalog():
    cat=setup_sample()

    for i in range(len(cat.items)):
        assert type(cat.items[i]) == "<class 'checkout.items.Book>" or "<class 'checkout.items.Dvd>"

def test_member_name_and_ids():
    cat=setup_sample()
    test_member=Member("Testing")

    test_member.borrow("B1")

    assert test_member.name == "Testing"
    assert test_member.current_ids[0] == "B1"
