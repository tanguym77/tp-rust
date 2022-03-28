#![allow(non_snake_case)]

pub fn conversion(nb:i32) {

    let fahr;
    fahr = nb * 9/5 + 32;
    println!("{} Degré correspond à {} Fahrenheit", nb, fahr);
}