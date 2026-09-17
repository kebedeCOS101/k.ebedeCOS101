// Rust program to read the height of a person
// and then print if person is tall, short,
// or average height person

use std::io::{self, Write};

fn main() {
    let height: f32 = loop {
        let mut input = String::new();

        print!("\nEnter Your Height (in centimetres): ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).expect("Not a valid string");

        match input.trim().parse::<f32>() {
            Ok(h) if h >= 50.0 && h <= 260.0 => break h,
            Ok(_) => println!("That height is out of range."),
            Err(_) => println!("Please enter a number."),
        }
    };

    if height > 195.0 {
        println!("You are very tall");
    } else if height > 170.0 {
        println!("You are tall");
    } else if height >= 150.0 {
        println!("You are of average height");
    } else {
        println!("You are short");
    }
}