use std::io;
fn main() {
    loop {
    println!("input your age");
    let mut age= String::new();
    io::stdin().read_line(&mut age).expect("failed to read input");
    let mut age:f32 = age.trim().parse().expect("failed to read input");
    if age >= 40.0
    {println!("1,560,000");}
    else if age >= 28.0 && age < 39.0
    {
    println!("1,480,000");
    }
    else if age < 28.0
    {
        println!("1,300,000");
    }
    if age == 0.0{
    println!("your not in this company then");}
   
    } 
}
