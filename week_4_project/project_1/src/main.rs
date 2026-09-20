use std::io;
//Assigning A as a mutable variable and making loop for float variable

fn main() {
    let a: f32 = loop{
    println!("Assign A for calculations");
    let mut a = String::new();
    io::stdin()
    .read_line(&mut a)
    .expect("failed to read input");
    match a.trim().parse::<f32>(){
     Ok(num) => break num,
     Err(_) => println!("This isnt a number try again"),
    }
};
//assigning b as a mutable variable and creating loop for float variable
    let b: f32 = loop{
    println!("Assign B for calculations");
    let mut b = String::new();
    io::stdin()
    .read_line(&mut b)
    .expect("failed to read input");
    match b.trim().parse::<f32>(){
     Ok(num) => break num,
     Err(_) => println!("This isnt a number try again"),
    }
};
//assigning c as a mutable variable and creating loop for float variable
    let c: f32 = loop{
    println!("Assign C for calculations");
    let mut C = String::new();
    io::stdin()
    .read_line(&mut C)
    .expect("failed to read input");
    match C.trim().parse::<f32>(){
     Ok(num) => break num,
     Err(_) => println!("This isnt a number try again"),
    }
};
//inputong formula
let d = b*b - 4.0*a*c;
println!("{} is the quadratic root of the equation",d );
  }
