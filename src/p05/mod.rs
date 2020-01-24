/// Reverse a list
/// input:
///   let xs = vec![1, 1, 2, 3, 5, 8]
///   reverse(xs)
/// output:
///   vec[8, 5, 3, 2, 1, 1]

pub fn reverse<T: Clone>(xs: &Vec<T>) -> Vec<T> {
    let mut mut_xs = xs.clone();
    mut_xs.reverse();
   return mut_xs.to_vec();
}

pub fn reverse_recursive<T: Clone>(xs: &Vec<T>) -> Vec<T> {
    helper(xs, vec![])
}

fn helper<T: Clone>(current_xs: &Vec<T>, accumulator: Vec<T>) -> Vec<T> {
    if current_xs.is_empty() {
        return accumulator
    } else {
        let head = current_xs.get(0);
        let tail = current_xs.clone().into_iter().skip(1).collect();

        let mut mut_head = vec![head.unwrap().clone()];
        let mut mut_accumulator = accumulator.clone();
        mut_head.append(&mut mut_accumulator);

        let new_accumulator = mut_head.clone();
        helper(&tail, new_accumulator)
    }
}

/// ------------[ Tests ] ---------------------
#[cfg(test)]
mod tests {
    use crate::p05::{reverse, reverse_recursive};

    #[test]
    fn reverse_valid_vec() {
        let input = vec![1, 2, 3, 4];
        let actual = reverse(&input);
        assert_eq!(actual, vec![4, 3, 2, 1])
    }

    #[test]
    fn reverse_with_empty_input() {
        let input: Vec<usize> = vec![];
        let actual = reverse(&input);
        assert_eq!(actual, vec![])
    }

    #[test]
    fn reverse_recursive_valid_input() {
        let input = vec![1, 2, 3, 4];
        let actual = reverse_recursive(&input);
        assert_eq!(actual, vec![4, 3, 2, 1])
    }

    #[test]
    fn reverse_recursive_with_empty_input() {
        let input: Vec<usize> = vec![];
        let actual = reverse_recursive(&input);
        assert_eq!(actual, vec![])
    }
}
