#![allow(non_snake_case)]

fn main() {
    // Programme Palindrome
    let mot: &str = "TAT";
    palindrome(mot);
}

fn palindrome(mot: &str) {
    let inverse = mot.chars().rev().collect::<String>();
    if mot == inverse {
        println!("{} est un palindrome", mot);
    } else {
        println!("{} n'est pas un palindrome", mot);
    }
}