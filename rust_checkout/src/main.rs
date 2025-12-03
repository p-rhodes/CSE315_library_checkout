mod checkout;
use checkout::items::Book;
use checkout::items::Dvd;
use checkout::catalog::Catalog;
use checkout::member::Member;

fn main() {
    let mut cat=Catalog::new();

    cat.add(Box::new(Book::new(String::from("B1"), String::from("Rust for Humans"))));
    cat.add(Box::new(Book::new(String::from("B2"), String::from("Pythonic Patterns"))));
    cat.add(Box::new(Book::new(String::from("D1"), String::from("Taking Flight"))));
    cat.add(Box::new(Book::new(String::from("D2"), String::from("Patterns in Motion"))));
    cat.add(Box::new(Dvd::new(String::from("C1"), String::from("Lord of the Flies"))));

    let mut clark=Member::new(String::from("Clark"));
    clark.borrow(String::from("B1"));
    clark.borrow(String::from("B1")); //attempt to borrow twice to prove no duplicates
    clark.borrow(String::from("D1"));
    clark.borrow(String::from("C1"));

    println!("{} has borrowed: {:?}", clark.name(), clark.borrowed_ids());

    println!("Printing details for Member <{}>", clark.name());
    for (id_val, title, days) in cat.details_for(&clark.borrowed_ids()) {
        println!("  -> {} - {} ({} days)", id_val, title, days);
    }

    clark.return_item(String::from("B1"));
    println!("{} has borrowed: {:?}", clark.name(), clark.borrowed_ids());
}
