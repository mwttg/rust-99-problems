/// Find the last element of a list
/// input:
///   let xs = vec![1, 1, 2, 3, 5, 8]
///   find_last_functional(xs)
/// output:
///   Some(8)

pub fn find_last_recursive(xs: &Vec<i32>) -> Option<i32> {
    return helper(xs, &None);
}

fn helper(xs: &Vec<i32>, accumulator: &Option<i32>) -> Option<i32> {
    if xs.is_empty() {  // unfortunately I can't use pattern matching with Vec deconstruction
        return accumulator.clone();
    } else {
        let head = xs[0];
        let tail = xs[1..].to_vec();
        helper(&tail, &Some(head))
    }
}

pub fn find_last_functional(xs: &Vec<i32>) -> Option<i32> {
    return xs.last().cloned();
}

/// ------------[ Tests ] ---------------------
#[cfg(test)]
mod tests {
    use crate::p01::find_last_functional;
    use crate::p01::find_last_recursive;

    #[test]
    fn test_find_last_functional_valid_input() {
        let input = vec![1, 2, 3];
        let actual = find_last_functional(&input);
        assert_eq!(actual, Some(3))
    }

    #[test]
    fn test_find_last_functional_empty_input() {
        let input = vec![];
        let actual = find_last_functional(&input);
        assert_eq!(actual, None)
    }

    #[test]
    fn find_last_recursive_empty_input() {
        let input = vec![];
        let actual = find_last_recursive(&input);
        assert_eq!(actual, None)
    }

    #[test]
    fn find_last_recursive_input_of_size_one() {
        let input = vec![99];
        let actual = find_last_recursive(&input);
        assert_eq!(actual, Some(99))
    }

    #[test]
    fn find_last_recursive_valid_input() {
        let input = vec![1, 2, 3];
        let actual = find_last_recursive(&input);
        assert_eq!(actual, Some(3))
    }
}
