//made by Parker Rhodes

mod checkout;
use checkout::items::{Book, Dvd, Magazine};
use checkout::catalog::Catalog;
use checkout::member::Member;

fn main() {
    let mut cat=Catalog::new();

    cat.add(Box::new(Book::new(String::from("B1"), String::from("Rust for Humans"))));
    cat.add(Box::new(Book::new(String::from("B2"), String::from("Pythonic Patterns"))));
    cat.add(Box::new(Book::new(String::from("B3"), String::from("Taking Flight"))));
    cat.add(Box::new(Book::new(String::from("B4"), String::from("Patterns in Motion"))));
    cat.add(Box::new(Dvd::new(String::from("D1"), String::from("Lord of the Flies"))));
    cat.add(Box::new(Magazine::new(String::from("M1"), String::from("Magic Code Mania"))));

    let mut clark=Member::new(String::from("Clark"));
    clark.borrow(String::from("B1"));
    clark.borrow(String::from("D1"));
    clark.borrow(String::from("B2"));
    clark.borrow(String::from("M1"));

    println!("{} has borrowed: {:?}", clark.name(), clark.borrowed_ids());

    println!("Printing details for Member <{}>", clark.name());
    for (id_val, title, days) in cat.details_for(&clark.borrowed_ids()) {
        println!("  -> {} - {} ({} days)", id_val, title, days);
    }

    clark.return_item(String::from("B1"));
    println!("{} has borrowed: {:?}", clark.name(), clark.borrowed_ids());
}

//BEGIN UNIT TESTS------------------------------->

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_and_get() {
        let mut cat: Catalog=Catalog::new();
        cat.setup_sample();

        for i in 0..cat.items.len() {
            println!("ID: {}", cat.items[i].get_id());
        }

        assert_eq!(cat.get(String::from("B1")).get_name(), String::from("Rust for Humans"));
    }

    #[test]
    fn test_duplicate_id_rejected() {
        let mut cat: Catalog=Catalog::new();
        cat.setup_sample();

        let cat_size_start: usize=cat.items.len();

        cat.add(Box::new(Book::new(String::from("B1"), String::from("Rust for Humans"))));

        assert_eq!(cat_size_start, cat.items.len());
    }

    #[test]
    fn test_member_borrow_and_return() {
        let mut cat: Catalog=Catalog::new();
        cat.setup_sample();

        let mut clark: Member=Member::new(String::from("Clark"));

        clark.borrow(String::from("B1"));
        assert_eq!(clark.borrowed_ids()[0], "B1");

        clark.return_item(String::from("B1"));
        assert_eq!(clark.borrowed_ids().len(), 0);
    }

    #[test]
    fn test_member_cannot_borrow_twice() {
        let mut m=Member::new(String::from("x"));

        m.borrow(String::from("K1"));
        m.borrow(String::from("K1"));

        assert_eq!(m.borrowed_ids().len(), 1);
    }

    #[test]
    fn test_member_limit_enforced() {
        let mut m=Member::new(String::from("x"));

        for i in 0..6 {
            let id=format!("K{i}");
            m.borrow(id);
        }

        assert_eq!(m.borrowed_ids().len(), 5);
    }

    #[test]
    fn test_details_lines() {
        let mut cat=Catalog::new();
        cat.setup_sample();

        let mut m=Member::new(String::from("y"));

        m.borrow(String::from("B2"));

        let test_vec: Vec<(String, String, u8)>=cat.details_for(&m.borrowed_ids());
        assert_eq!(test_vec[0], (String::from("B2"), String::from("Pythonic Patterns"), 7));
    }

    #[test]
    fn test_member_name_and_ids() {
        let mut m=Member::new(String::from("x"));

        m.borrow(String::from("B1"));

        assert_eq!(m.name(), "x");
        assert_eq!(m.borrowed_ids()[0], "B1");
    }

    #[test]
    fn test_add_bad_item_should_pass() {
        let mut cat=Catalog::new();

        let bad_book=Box::new(Book::new(String::from(""), String::from("")));
        cat.add(bad_book);

        assert_eq!(cat.items.len(), 0);
    }

}
