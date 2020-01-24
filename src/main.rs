mod p01;
mod p02;
mod p03;
mod p04;
mod p05;

fn main() {
    println!("99 Problems in Rust");
    println!("===================");

    println!("Problem 01 - Find the last element of a list. Input 'vec![1, 1, 2, 3, 5, 8]'");
    let p01_input = vec![1, 1, 2, 3, 5, 8];
    let p01_result_recursive = p01::find_last_recursive(&p01_input);
    let p01_result_functional = p01::find_last_functional(&p01_input);
    println!("The result (recursive) is:  {:?}", p01_result_recursive);
    println!("The result (functional) is: {:?}", p01_result_functional);
    println!("-------------------");

    println!("Problem 02 - Find the last but one element of a list. Input 'vec![1, 1, 2, 3, 5, 8]'");
    let p02_input = vec![1, 1, 2, 3, 5, 8];
    let p02_result = p02::penultimate(&p02_input);
    println!("The result is: {:?}", p02_result);
    println!("-------------------");

    println!("Problem 03 - Find the Kth element of a list. Input 'vec![1, 1, 2, 3, 5, 8]'");
    let p03_input = vec![1, 1, 2, 3, 5, 8];
    let p03_result = p03::nth(2, &p03_input);
    let p03_result_recursive = p03::nth_recursive(2, &p03_input);
    println!("The result is: {:?}", p03_result);
    println!("The result is: {:?}", p03_result_recursive);
    println!("-------------------");

    println!("Problem 04 - Find the number of elements of a list. Input 'vec![1, 1, 2, 3, 5, 8]'");
    let p04_input = vec![1, 1, 2, 3, 5, 8];
    let p04_result = p04::length(&p04_input);
    let p04_result_recursive = p04::length_recursive(&p04_input);
    println!("The result is: {:?}", p04_result);
    println!("The result is: {:?}", p04_result_recursive);
    println!("-------------------");

    println!("Problem 05 - Reverse a list. Input 'vec![1, 1, 2, 3, 5, 8]'");
    let p05_input = vec![1, 1, 2, 3, 5, 8];
    let p05_result = p05::reverse(&p05_input);
    println!("The result is: {:?}", p05_result);
    println!("-------------------");
}
