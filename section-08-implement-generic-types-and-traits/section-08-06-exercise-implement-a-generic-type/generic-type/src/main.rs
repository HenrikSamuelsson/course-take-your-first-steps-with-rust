struct Jar<T> {
    value: T,
}

impl<T> Jar<T> {
    pub fn new(value: T) -> Self {
        Jar { value }
    }
}
fn main() {
    assert_eq!(Jar::new(42).value, 42);
    assert_eq!(Jar::new(3.14).value, 3.14);
    assert_eq!(Jar::new("Foo").value, "Foo");
    assert_eq!(Jar::new(String::from("Bar")).value, String::from("Bar"));
    assert_eq!(Jar::new(true).value, true);
    assert_eq!(Jar::new(-12).value, -12);
    assert_eq!(Jar::new(Some("text")).value, Some("text"));
}
