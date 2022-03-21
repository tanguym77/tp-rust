#![allow(non_snake_case)]

fn main() {
    // Programme Fermat
    let a=13;
    fermat(a);
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