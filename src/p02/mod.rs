/// Find the last but one element of a list
/// input:
///   let xs = vec![1, 1, 2, 3, 5, 8]
///   penultimate(xs)
/// output:
///   Some(5)

pub fn penultimate<T: Clone>(xs: &Vec<T>) -> Option<T> {
    let length = xs.len();
    return if length <= 1 {
        None
    } else {
        let position = length - 1 - 1; // counting from zero; penultimate element
        let result = xs.into_iter().nth(position);
        result.cloned()
    }
}

/// ------------[ Tests ] ---------------------
#[cfg(test)]
mod tests {
    use crate::p02::penultimate;

    #[test]
    fn penultimate_valid_input_int() {
        let input = vec![1, 2, 3];
        let actual = penultimate(&input);
        assert_eq!(actual, Some(2))
    }

    #[test]
    fn penultimate_valid_input_string() {
        let input = vec!["1", "2", "3"];
        let actual = penultimate(&input);
        assert_eq!(actual, Some("2"))
    }

    #[test]
    fn penultimate_valid_empty_input() {
        let input: Vec<i32> = Vec::new();
        let actual = penultimate(&input);
        assert_eq!(actual, None)
    }

    #[test]
    fn penultimate_valid_input_of_size_one() {
        let input = vec![5];
        let actual = penultimate(&input);
        assert_eq!(actual, None)
    }
}
