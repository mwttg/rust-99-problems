# Rust 99 Problems

This is a repository for playing around (learning) Rust.
The task are from: 
* [Scala 99 Problems][scala-99-problems]

Some resources for Rust:
* [The Programming Language Rust][rust-book]
* [Rust by Example][rust-by-example]
* [Rust Style Guidelines][rust-style-guide]

# Problems
## P01 (*) Find the last element of a list
Input:
```
find_last(vec![1, 1, 2, 3, 5, 8]);
```
Output:
```
Some(8)
```
[solution](src/p01/mod.rs)

## P02 (*) Find the last but one element of a list.
Input:
```
penultimate(vec![1, 1, 2, 3, 5, 8]);
```
Output:
```
Some(5)
```
[solution](src/p02/mod.rs)

## P03 (*) Find the Kth element of a list.
By convention, the first element in the list is element 0.

Input:
```
nth(2, vec![1, 1, 2, 3, 5, 8])
```
Output:
```
Some(2)
```
[solution](src/p03/mod.rs)

## P04 (*) Find the number of elements of a list.
Input:
```
length(vec![1, 1, 2, 3, 5, 8])
```
Output:
```
6
```
[solution](src/p04/mod.rs)

## P05 (*) Reverse a list.
Input:
```
reverse(vec![1, 1, 2, 3, 5, 8])
```
Output:
```
vec![8, 5, 3, 2, 1, 1]
```
[solution](src/p05/mod.rs)

[rust-book]: https://doc.rust-lang.org/book/title-page.html
[rust-by-example]: https://doc.rust-lang.org/rust-by-example/index.html
[rust-style-guide]: https://doc.rust-lang.org/1.0.0/style/README.html
[scala-99-problems]: http://aperiodic.net/phil/scala/s-99/
