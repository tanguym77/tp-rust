#![allow(non_snake_case)]

fn main() {
    // Programme Euclide
    let a = 24;
    let b = 36;
    euclide(a,b);
}

fn euclide(a: u32, b: u32) {
    let mut a = a;
    let mut b = b;

    while a!=b {
        if a > b {
            a = a - b;
        }else {
            b = b -a;
        }
    }
    println!("Le PGCD est {}", a)
}