fn my_loop() -> i32 {
    let mut a = 1;
    loop {
        a += 1;
        if a == 10 {
            break 10;
        }
    }
}

fn my_while() -> i32 {
    let mut b = 1;
    while b < 10 {
        b += 1;
    }
    b
}

fn my_for() -> i32 {
    let mut c = 0;
    for i in 0..10 {
        c += 1;
    }
    c
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_my_loop() {
        let a = my_loop();
        assert_eq!(a, 10);
    }
    #[test]
    fn test_my_while() {
        let b = my_while();
        assert_eq!(b, 10);
    }
    #[test]
    fn test_my_for() {
        let c = my_for();
        assert_eq!(c, 10);
    }
}
