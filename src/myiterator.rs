pub struct Stepper {
    curr: i32,
    step: i32,
    max: i32,
}

impl Iterator for Stepper {
    type Item = i32;

    fn next(&mut self) -> Option<i32> {
        if self.curr >= self.max {
            return None;
        }
        let res = self.curr + self.step;
        self.curr += self.step;
        Some(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_stepper_next() {
        let mut stepper = Stepper {
            curr: 5,
            step: 2,
            max: 20,
        };
        assert_eq!(stepper.next(), Some(7));
    }
}
