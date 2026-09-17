// Rust program to determine age pass

use std::io::{self, Write};

fn main() {
    let mut input1 = String::new();

    print!("Enter your name: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let name = input1.trim();

    let age: u32 = loop {
        let mut buf = String::new();
        print!("Enter your age: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut buf).expect("Not a valid string");
        match buf.trim().parse() {
            Ok(n) => break n,
            Err(_) => println!("Please enter a whole number."),
        }
    };

    if age >= 18 {
        println!("Welcome to the party {}!", name);
    } else {
        println!("Oops {}, you are not of age to enter the party", name);
    }
}