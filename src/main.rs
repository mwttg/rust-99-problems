mod p01;
mod p02;

fn main() {
    println!("99 Problems in Rust");
    println!("===================");

    println!("Problem 01 - Find the last element of a list. Input 'vec![1, 1, 2, 3, 5, 8]'");
    let p01_input = vec![1, 1, 2, 3, 5, 8];
    let p01_result_recursive = p01::find_last_recursive(p01_input.clone());
    let p01_result_functional = p01::find_last_functional(p01_input.clone());
    println!("The result (recursive) is:  {:?}", p01_result_recursive);
    println!("The result (functional) is: {:?}", p01_result_functional);
    println!("-------------------");

    println!("Problem 02 - Find the last but one element of a list. Input 'vec![1, 1, 2, 3, 5, 8]'");
    let p01_input = vec![1, 1, 2, 3, 5, 8];
    println!("-------------------");
}
