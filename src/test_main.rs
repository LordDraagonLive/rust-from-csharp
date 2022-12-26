#[cfg(test)]
mod tests {
    use crate::Person;

    #[test]
    fn test() {
        assert_eq!(1 + 2, 3);
    }

    ///
    /// example of a test for a struct method in Rust
    /// 
    #[test]
    fn test_person() {
        let person = Person::create("John", 29);
        assert_eq!(person.can_drink(), true);
    }
}
