/// Find the number of elements of a list
/// input:
///   let xs = vec![1, 1, 2, 3, 5, 8]
///   length(xs)
/// output:
///   6

pub fn length<T: Clone>(xs: &Vec<T>) -> usize {
    return xs.len();
}

pub fn length_recursive<T: Clone>(xs: &Vec<T>) -> usize {
    return helper(xs, 0)
}

fn helper<T: Clone>(current_xs: &Vec<T>, accumulator: usize) -> usize {
    if current_xs.is_empty() {
        accumulator
    } else {
        let tail = current_xs.clone().into_iter().skip(1).collect();
       return  helper(&tail, accumulator + 1)
    }
}


/// ------------[ Tests ] ---------------------
#[cfg(test)]
mod tests {
    use crate::p04::{length, length_recursive};

    #[test]
    fn length_valid_input() {
        let input = vec![1, 2, 3, 4];
        let actual = length(&input);
        assert_eq!(actual, 4)
    }

    #[test]
    fn length_empty_input() {
        let input: Vec<i32> = vec![];
        let actual = length(&input);
        assert_eq!(actual, 0)
    }

    #[test]
    fn length_recursive_valid_input() {
        let input = vec![1, 2, 3, 4];
        let actual = length_recursive(&input);
        assert_eq!(actual, 4)
    }

    fn length_recursive_empty_input() {
        let input: Vec<i32> = vec![];
        let actual = length_recursive(&input);
        assert_eq!(actual, 0)
    }
}
