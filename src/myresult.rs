pub enum Res<T, E> {
    Thing(T),
    Error(E),
}

pub fn divide(a: i32, b: i32) -> Res<i32, String> {
    if b == 0 {
        Res::Error("division by zero, it is not possible to divide".to_string())
    } else {
        Res::Thing(a / b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divid_ok() {
        let a = divide(10, 2);
        match a {
            Res::Thing(number) => assert_eq!(5, number),
            _ => panic!("failed test"),
        }
    }
    #[test]
    fn test_divid_err() {
        let a = divide(10, 0);
        match a {
            Res::Thing(_) => panic!("failed test"),
            Res::Error(err) => assert_eq!(
                "division by zero, it is not possible to divide".to_string(),
                err
            ),
        }
    }
    #[test]
    fn test_divid_if_let() {
        let a = divide(2, 1);
        if let Res::Thing(c) = a {
            assert_eq!(2, c);
        }
    }
}
