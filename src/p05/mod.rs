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

/// ------------[ Tests ] ---------------------
#[cfg(test)]
mod tests {
    use crate::p05::{reverse};

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
}
