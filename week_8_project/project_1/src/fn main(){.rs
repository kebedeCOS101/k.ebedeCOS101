fn main(){
    println!("What is your profession input these values to select \n 0 for Office Admin \n 1 for academic \n 2 for lawyer \n 3 for teacher");
    let vprof = vec!["Office Admin","academic","lawyer","teacher"];
    let mut input1 = String::new();
    println!("Enter an index of 0-3");
    std::io::stdin().read_line(&mut input1).expect("Failed to read input");
    let index:usize = input1.trim().parse().expect("failed to read input");
    let _string: String = vprof[index].to_string();
    if index == 0 {
        println!("Office admin chosen \n Choose your category using index \n 0 for intern\n 1 for administrator \n 2 for Senior admiistrator\n 3 for Office manager\n 4 for Director \n 5 for CEO!!");

        let years = vec!["Intern","administrator","Senior administrator","Office manager","director","CEO"];
        let mut inputyrs = String::new();
        println!("Enter an index of 0-5");
        std::io::stdin().read_line(&mut inputyrs).expect("failed to read input");
        let index:usize = inputyrs.trim().parse().expect("failed to read input");
        let chosen_years: String = years[index].to_string();
        if index== 0{
            println!("how many years have you serve");
            let mut yearsinput = String::new();
        std::io::stdin().read_line(&mut yearsinput).expect("Failed to read input");
        let yearsofexp: i32 = yearsinput.trim().parse().expect("Failed to read input");
        if yearsofexp >= 1 &&yearsofexp <= 2 {
            println!("valid: intern has an Aps level of 1-2");
        }
        else{ 
            println!("invalid: an intern should have 1 to 2 years of experience");
            }
        
    }
    else if index == 1{ println!("how many years have you serve");
            let mut yearsinput = String::new();
        std::io::stdin().read_line(&mut yearsinput).expect("Failed to read input");
        let yearsofexp: i32 = yearsinput.trim().parse().expect("Failed to read input");
        if yearsofexp >= 3 &&yearsofexp <= 4 {
            println!("valid: administrator has an Aps level of 3-4");
        }
        else{ println!("invalid: an administrator should have 3-4 years of experience")
            }

    }
    else if index == 2 {
         println!("how many years have you serve");
            let mut yearsinput = String::new();
        std::io::stdin().read_line(&mut yearsinput).expect("Failed to read input");
        let yearsofexp: i32 = yearsinput.trim().parse().expect("Failed to read input");
        if yearsofexp >= 5 &&yearsofexp <= 7 {
            println!("valid: administrator has an Aps level of 5-6 ");
        }
        else{ 
            println!("invalid: an administrator should have 5-6 years of experience");
        }
    }
    else if index == 3 {
         println!("how many years have you serve");
            let mut yearsinput = String::new();
        std::io::stdin().read_line(&mut yearsinput).expect("Failed to read input");
        let yearsofexp: i32 = yearsinput.trim().parse().expect("Failed to read input");
        if yearsofexp >=8  &&yearsofexp <= 10 {
            println!("valid: administrator has an Aps level of 8-10");
        }
        else{ 
            println!("invalid: an administrator should have 8-10 years of experience");}
    }
}