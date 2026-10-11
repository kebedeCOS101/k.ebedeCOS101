// Project: The Shape Calculator
// Built only from what the Week 7 slides teach: functions that return a value,
// reading input with read_line, and if / else decisions.

use std::io;

// Practice 3: a function that returns a value
fn get_pi() -> f64 {
    let a: f64 = 22.0;
    let b: f64 = 7.0;
    let c: f64 = a / b;
    return c;
}

// Trapezium, area = height / 2 * (base1 + base2)
fn trapezium_area() -> f64 {
    let mut input1 = String::new();
    println!("Enter base 1:");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let base1: f64 = input1.trim().parse().expect("Invalid input");

    let mut input2 = String::new();
    println!("Enter base 2:");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let base2: f64 = input2.trim().parse().expect("Invalid input");

    let mut input3 = String::new();
    println!("Enter height:");
    io::stdin().read_line(&mut input3).expect("Failed to read input");
    let height: f64 = input3.trim().parse().expect("Invalid input");

    let area: f64 = height / 2.0 * (base1 + base2);
    return area;
}

// Rhombus, area = 1/2 * diagonal1 * diagonal2
// The numbers are f64, so 1/2 is written 1.0 / 2.0
fn rhombus_area() -> f64 {
    let mut input1 = String::new();
    println!("Enter diagonal 1:");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let diagonal1: f64 = input1.trim().parse().expect("Invalid input");

    let mut input2 = String::new();
    println!("Enter diagonal 2:");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let diagonal2: f64 = input2.trim().parse().expect("Invalid input");

    let area: f64 = 1.0 / 2.0 * diagonal1 * diagonal2;
    return area;
}

// Parallelogram, area = base * altitude
fn parallelogram_area() -> f64 {
    let mut input1 = String::new();
    println!("Enter base:");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let base: f64 = input1.trim().parse().expect("Invalid input");

    let mut input2 = String::new();
    println!("Enter altitude:");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let altitude: f64 = input2.trim().parse().expect("Invalid input");

    let area: f64 = base * altitude;
    return area;
}

// Cube, surface area = 6 * side * side
fn cube_surface_area() -> f64 {
    let mut input1 = String::new();
    println!("Enter side:");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let side: f64 = input1.trim().parse().expect("Invalid input");

    let area: f64 = 6.0 * side * side;
    return area;
}

// Cylinder, volume = pi * radius * radius * height
fn cylinder_volume() -> f64 {
    let mut input1 = String::new();
    println!("Enter radius:");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let radius: f64 = input1.trim().parse().expect("Invalid input");

    let mut input2 = String::new();
    println!("Enter height:");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let height: f64 = input2.trim().parse().expect("Invalid input");

    let pi: f64 = get_pi();
    let volume: f64 = pi * radius * radius * height;
    return volume;
}

fn main() {
    println!("Welcome to the Shape Calculator");
    println!("1. Trapezium (area)");
    println!("2. Rhombus (area)");
    println!("3. Parallelogram (area)");
    println!("4. Cube (surface area)");
    println!("5. Cylinder (volume)");

    // one read for the choice
    let mut input = String::new();
    println!("Enter your choice (1 to 5):");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let choice: i32 = input.trim().parse().expect("Invalid input");

    // then one function per shape
    if choice == 1 {
        let area = trapezium_area();
        println!("Area of the trapezium = {}", area);
    } else if choice == 2 {
        let area = rhombus_area();
        println!("Area of the rhombus = {}", area);
    } else if choice == 3 {
        let area = parallelogram_area();
        println!("Area of the parallelogram = {}", area);
    } else if choice == 4 {
        let area = cube_surface_area();
        println!("Surface area of the cube = {}", area);
    } else if choice == 5 {
        let volume = cylinder_volume();
        println!("Volume of cylinder = {}", volume);
    } else {
        println!("Invalid choice. Run the program again choose 1 to 5.");
    }
}