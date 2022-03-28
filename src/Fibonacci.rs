#![allow(non_snake_case)]

pub fn fibo(n:u32) {
    println!("{}", fib(n));
}

pub fn fib(n: u32) -> u32 {
    match n {
        0 => 1,
        1 => 1,
        _ => fib(n - 1) + fib(n - 2),
    }
}