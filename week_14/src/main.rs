use std::io::Read;
use std::io;
fn main() {
    println!("What level of access do you have");
    println!("Administrator");
    println!("Project manager");
    println!("Employee");
    println!("Customer");
    println!("Vendor");
    println!("Input password Side note the pass word is the name of the level of access");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    if input1.trim() == "Administrator"
    {
    println!("Access granted");    
    let  file = std::fs::File::open("globacom_dbase.sql");
    let mut contents = String::new();
    file.expect("failed  to read files").read_to_string(&mut contents).unwrap();// opening file based on access
    print!("{}",contents);
}

    else if input1.trim() == "administrator"{ 
    println!("Access granted");
    let  file = std::fs::File::open("globacom_dbase.sql");
    let mut contents = String::new();
    file.expect("failed  to read files").read_to_string(&mut contents).unwrap();
    print!("{}",contents);
}
 else if input1.trim() == "Project manager"{ 
    println!("Access granted");
    let  file = std::fs::File::open("project_tb.sql");
    let mut contents = String::new();
    file.expect("failed  to read files").read_to_string(&mut contents).unwrap();
    print!("{}",contents);

 }
  else if input1.trim() == "project manager"{ 
    println!("Access granted");
    let  file = std::fs::File::open("project_tb.sql");
    let mut contents = String::new();
    file.expect("failed  to read files").read_to_string(&mut contents).unwrap();
    print!("{}",contents);

 }

 else if input1.trim() == "Employee"{
     println!("Access granted");
    let  file = std::fs::File::open("staff_tb.sql");
    let mut contents = String::new();
    file.expect("failed  to read files").read_to_string(&mut contents).unwrap();
    print!("{}",contents);

 }
 else if input1.trim() == "employee"{
     println!("Access granted");
    let  file = std::fs::File::open("staff_tb.sql");
    let mut contents = String::new();
    file.expect("failed  to read files").read_to_string(&mut contents).unwrap();
    print!("{}",contents);

 }
else if input1.trim() == "Customer"{
     println!("Access granted");
    let  file = std::fs::File::open("customer.sql");
    let mut contents = String::new();
    file.expect("failed  to read files").read_to_string(&mut contents).unwrap();
    print!("{}",contents);
 }
 else if input1.trim() == "customer"{
     println!("Access granted");
    let  file = std::fs::File::open("customer.sql");
    let mut contents = String::new();
    file.expect("failed  to read files").read_to_string(&mut contents).unwrap();
    print!("{}",contents);
 }
else if  input1.trim() == "Vendor"{
     println!("Access granted");
    let  file = std::fs::File::open("dataplan_tb.sql");
    let mut contents = String::new();
    file.expect("failed  to read files").read_to_string(&mut contents).unwrap();
    print!("{}",contents);
 }
 else if  input1.trim() == "vendor"{
     println!("Access granted");
    let  file = std::fs::File::open("dataplan_tb.sql");
    let mut contents = String::new();
    file.expect("failed  to read files").read_to_string(&mut contents).unwrap();
    print!("{}",contents);
 }
 else {
     println!("Access failed"); //catch all
 }

}

