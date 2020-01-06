mod p01;

fn main() {
    println!("99 Problems in Rust");
    println!("===================");

    println!("Problem 01 - find_last: Input 'vec![1, 1, 2, 3, 5, 8]'");
    let result = p01::find_last_functional(vec![1, 1, 2, 3, 5, 8]);
    println!("The result (functional) is: {:?}", result);
    println!("-------------------");
}
