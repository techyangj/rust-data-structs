#[derive(Debug)]
pub struct LinkedList<T> {
    data: T,
    next: Option<Box<LinkedList<T>>>,
}

impl<T: std::ops::AddAssign> LinkedList<T> {
    pub fn add_up(&mut self, n: T) {
        self.data += n;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_linkedlist() {
        let mut list = LinkedList {
            data: 3,
            next: Some(Box::new(LinkedList {
                data: 2,
                next: None,
            })),
        };
        assert_eq!(list.next.unwrap().data, 2);
    }
    #[test]
    fn test_linkedlist_add_up() {
        let mut list = LinkedList {
            data: 3,
            next: Some(Box::new(LinkedList {
                data: 2,
                next: None,
            })),
        };
        if let Some(ref mut v) = list.next {
            v.add_up(10);
            assert_eq!(v.data, 12);
        }
    }
}
