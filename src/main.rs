#![allow(non_snake_case)]

fn main() {
    // Programme Euclide
    let a = 24;
    let b = 36;
    euclide(a,b);

    // Programme Fermat
    let a=13;
    fermat(a);

    // Programme Palindrome
    let mot: &str = "TAT";
    palindrome(mot);

    // Programme Conversion
    let nb = 25;
    Conversion(nb)
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

fn fermat(a: i32) {
    for i in 2..a-1 {
        if a%i==0 {
            println!("{} n'est pas premier", a);
            break
        }
        else {
            println!("{} est premier", a);
            break
        }
    }
}

fn palindrome(mot: &str) {
    let inverse = mot.chars().rev().collect::<String>();
    if mot == inverse {
        println!("{} est un palindrome", mot);
    } else {
        println!("{} n'est pas un palindrome", mot);
    }
}

fn Conversion(nb:i32) {
    let fahr;
    fahr = nb * 9/5 + 32;
    println!("{} Degré correspond à {} Fahrenheit", nb, fahr);
}