#[derive(Clone)]
pub struct Person {
    name: String,
    age: i32,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_person_clone() {
        let person = Person {
            name: "John".to_string(),
            age: 21,
        };
        let person2 = person.clone();
        assert_eq!(person2.age, 21);
    }
    #[test]
    fn test_copy() {
        let a = 2;
        let b = 3;
        assert_eq!(3, b);
    }
}
