/// Find the Kth element of a list
/// input:
///   let xs = vec![1, 1, 2, 3, 5, 8]
///   nth(2, xs)
/// output:
///   Some(2)

pub fn nth<T: Clone>(n: usize, xs: &Vec<T>) -> Option<T> {
    return xs.get(n).cloned();
}

pub fn nth_recursive<T: Clone>(n: usize, xs: &Vec<T>) -> Option<T> {
    helper(n, 0, xs, None)
}

fn helper<T: Clone>(n: usize, current_n: usize, current_xs: &Vec<T>, accumulator: Option<T>) -> Option<T> {
    if current_n > n {
        accumulator
    } else {
        let head = current_xs.get(0).cloned();
        let tail= current_xs.clone().into_iter().skip(1).collect();
        // let (head, tail) = current_xs.split_at(1);
        return helper(n, current_n + 1, &tail, head);
    }
}

/// ------------[ Tests ] ---------------------
#[cfg(test)]
mod tests {
    use crate::p03::{nth, nth_recursive};

    #[test]
    fn nth_valid_input() {
        let input = vec![1, 2, 3];
        let actual = nth(1, &input);
        assert_eq!(actual, Some(2))
    }

    #[test]
    fn nth_empty_vec() {
        let input: Vec<i32> = vec![];
        let actual = nth(1, &input);
        assert_eq!(actual, None)
    }

    #[test]
    fn nth_recursive_valid_input() {
        let input = vec![1, 2, 3];
        let actual = nth_recursive(1, &input);
        assert_eq!(actual, Some(2))
    }

    #[test]
    fn nth_recursive_empty_vec() {
        let input: Vec<i32> = vec![];
        let actual = nth_recursive(1, &input);
        assert_eq!(actual, None)
    }
}
