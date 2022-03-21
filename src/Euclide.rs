#![allow(non_snake_case)]

pub fn euclide() {
    let a = 24;
    let b = 36;

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