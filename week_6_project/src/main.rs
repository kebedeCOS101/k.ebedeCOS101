use std::io;
fn main () {
   
    let p:i32 = 3200;
    let f:i32 = 3000;
    let a:i32 = 2500;
    let e:i32 = 2000;
    let w:i32 = 2500;
    let mut choice = String::new();
    println!("Type p for poundo combo ");
    println!("\n f for fried rice and chicken");
    println!("\n a for amala and ewedu soup");
    println!("\n w for white rice and stew");
    println!("\n e for eba and egusi soup");
    println!("\n choose with letters to indicate");
    io::stdin()
    .read_line(&mut choice)
    .expect("failed to read input");
    let choice = choice.trim();
    let amount:i32;
    if choice == "p" {
        amount = p;
    }
    else if choice == "f"{
        amount = f;
    }
    else if choice == "e"{
    amount = e;
    }
    else if choice == "a"{
        amount = a;
    }
    else if choice == "w"{
        amount = w;
    } else {
    println!("invalid choose a suggested words");
    return;}
 println!("\n input your desired quantity");
 let mut quantity= String::new();
 io::stdin()
 .read_line(&mut quantity)
 .expect("failed to read this input");
 let quantity:i32 = quantity.trim().parse().expect("enter a number");
 let cost = quantity * amount;
 println!("your amount to pay is {}",cost );
 if cost > 10000{
 let discount = cost * 10/100;
 let finalcost = cost - discount;
 println!("since your purchase was above 10000 you recieved a 10% discount which brought your cost to {} ",finalcost);
}
}