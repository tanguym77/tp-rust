#![allow(non_snake_case)]

fn main() {
    // Programme Fibonacci
    let n = 10;
    println!("{}", fib(n));
}

fn fib(n: u32) -> u32 {
    match n {
        0 => 1,
        1 => 1,
        _ => fib(n - 1) + fib(n - 2),
    }
}