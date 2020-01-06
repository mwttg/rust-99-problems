/// Find the last element of a list
/// input:
///   let xs = vec![1, 1, 2, 3, 5, 8]
///   find_last_functional(xs)
/// output:
///   Some(8)
pub fn find_last_recursive(list: Vec<i32>) {}

pub fn find_last_functional(xs: Vec<i32>) -> Option<i32> {
    return xs.last().cloned();
}

#[cfg(test)]
mod tests {
    use crate::p01::find_last_functional;

    #[test]
    fn test_find_last_functional_valid_input() {
        let input = vec![1, 2, 3];
        let actual = find_last_functional(input);
        assert_eq!(actual, Some(3))
    }
    #[test]
    fn test_find_last_functional_empty_input() {
        let input = vec![];
        let actual = find_last_functional(input);
        assert_eq!(actual, None)
    }
}
