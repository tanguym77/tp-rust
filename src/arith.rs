fn main() {
    let a = 12;
    let b = 24;
    euclide(a,b);

    let a=13;
    fermat(a);
}

fn euclide(a: u32, b: u32) {
    let mut a = a;
    let mut b = b;

    while b!=0 {
        let t = b;
        b = a%b;
        a = t;
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

// Palindrome