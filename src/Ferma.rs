#![allow(non_snake_case)]

pub fn fermat(a:i32) {

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