#[derive(Debug)]
pub struct Person {
    name: String,
    age: i32,
    children: i32,
}

impl Person {
    pub fn print(self) -> String {
        format!(
            "name = {}, age = {}, children = {}",
            self.name, self.age, self.children
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_person_print() {
        let person = Person {
            name: "John".to_string(),
            age: 32,
            children: 32,
        };
        let person_print = person.print();
        println!("{}", person_print);
    }
}
