#![allow(non_snake_case)]

pub fn palin() {
    let mot: &str = "TAT";
    let inverse = mot.chars().rev().collect::<String>();
    if mot == inverse {
        println!("{} est un palindrome", mot);
    } else {
        println!("{} n'est pas un palindrome", mot);
    }
}