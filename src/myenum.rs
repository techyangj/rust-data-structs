#[derive(Debug)]
pub enum Color {
    Red(i32),
    Blue(i32),
    Green(i32),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_red_number() {
        let red = Color::Red(255);
        match red {
            Color::Red(number) => assert_eq!(255, number),
            Color::Green(_) => panic!("unexpected"),
            Color::Blue(_) => panic!("unexpected"),
        }
    }
}
