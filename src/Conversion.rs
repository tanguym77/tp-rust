#![allow(non_snake_case)]

fn main() {
    // Programme Conversion
    let nb = 25;
    Conversion(nb);
}

fn Conversion(nb:i32) {
    let fahr;
    fahr = nb * 9/5 + 32;
    println!("{} Degré correspond à {} Fahrenheit", nb, fahr);
}