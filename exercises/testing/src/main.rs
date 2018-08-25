fn main() {
    println!("Hello, world!");
}

#[test]
fn new_person_setup() {
    let p1 = Person::new();
    assert!(p1.is_valid());
    assert_eq!(p1.email, None);
    let p2 = Person::new();
    assert_ne!(p1.id, p2.id);
}
