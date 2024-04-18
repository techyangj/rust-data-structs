use std::fmt::Debug;

/*
冒泡排序
O(n^2)
在每次外部循环（外循环控制排序的轮数）开始时，将 sorted 设为 true，表示假设数组已经是有序的。然后在内部循环（内循环用于执行实际的排序操作）中，如果发现有任何一对相邻元素是逆序的，就将 sorted 设为 false，表示数组尚未完全有序。如果在内部循环结束时发现 sorted 仍然为 true，说明数组已经完全有序，可以提前退出外部循环，不必再继续执行排序操作，以提高效率。
 */
pub fn bubble_sort<T: PartialOrd + Debug>(v: &mut [T]) {
    for p in 0..v.len() {
        println!("{:?}", v);
        let mut sorted = true;
        for i in 0..(v.len() - 1) - p {
            if v[i] > v[i + 1] {
                v.swap(i, i + 1);
                sorted = false;
            }
        }
        if sorted {
            return;
        }
    }
}
// 合并排序
// n * log(n)
pub fn merge_sort<T: PartialOrd + Debug>(mut v: Vec<T>) -> Vec<T> {
    if v.len() <= 1 {
        return v;
    }

    let mut res = Vec::with_capacity(v.len());
    let b = v.split_off(v.len() / 2);
    let a = merge_sort(v);
    let b = merge_sort(b);

    let mut a_it = a.into_iter();
    let mut b_it = b.into_iter();
    let mut a_peek = a_it.next();
    let mut b_peek = b_it.next();

    loop {
        match a_peek {
            Some(ref a_val) => match b_peek {
                Some(ref b_val) => {
                    if b_val < a_val {
                        res.push(b_peek.take().unwrap());
                        b_peek = b_it.next();
                    } else {
                        res.push(a_peek.take().unwrap());
                        a_peek = a_it.next();
                    }
                }
                None => {
                    res.push(a_peek.take().unwrap());
                    res.extend(a_it);
                    return res;
                }
            },
            None => {
                if let Some(b_val) = b_peek {
                    res.push(b_val);
                }
                res.extend(b_it);
                return res;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_bubble_sort() {
        let mut v = vec![4, 6, 1, 8, 11, 13, 3];
        bubble_sort(&mut v);
        assert_eq!(v, vec![1, 3, 4, 6, 8, 11, 13]);
    }
    #[test]
    fn test_merge_sort() {
        let v = vec![4, 6, 1, 8, 11, 13, 3];
        let v = merge_sort(v);
        assert_eq!(v, vec![1, 3, 4, 6, 8, 11, 13]);
    }
}
