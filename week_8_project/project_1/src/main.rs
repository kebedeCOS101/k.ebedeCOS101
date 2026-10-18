fn main(){
    println!("What is your profession input these values to select \n 0 for Office Admin \n 1 for academic \n 2 for lawyer \n 3 for teacher");
    let vprof = vec!["Office Admin","academic","lawyer","teacher"];//Establishing Vector category
    let mut input1 = String::new();
    println!("Enter an index of 0-3");
    let mut inputyrs=String::new();
    std::io::stdin().read_line(&mut input1).expect("Failed to read input");
    let index:usize = input1.trim().parse().expect("failed to read input");
      if index > 3 {
        println!("Invalid profession, enter a number from 0 to 3");
        return;
    }
    let _string: String = vprof[index].to_string();
    let _string: String = vprof[index].to_string();
    if index == 0 {
        println!("Office admin chosen \n Choose your category using index \n 0 for intern\n 1 for administrator \n 2 for Senior admministrator\n 3 for Office manager\n 4 for Director \n 5 for CEO!!");
                let index:usize = inputyrs.trim().parse().expect("failed to read input");
        if index > 5 {
            println!("Invalid category, enter a number from 0 to 5");
            return;
        }//creating if else brackets for each category
        if index== 0{

        let years = vec!["Intern","administrator","Senior administrator","Office manager","director","CEO"];
        let mut inputyrs = String::new();
        println!("Enter an index of 0-5");
        std::io::stdin().read_line(&mut inputyrs).expect("failed to read input");
        let index:usize = inputyrs.trim().parse().expect("failed to read input");
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
            println!("valid: administrator has an Aps level of 5-7 ");
        }
        else{ 
            println!("invalid: an administrator should have 5-7 years of experience");
        }
      }
      else if index == 3 {
         println!("how many years have you serve");
            let mut yearsinput = String::new();
        std::io::stdin().read_line(&mut yearsinput).expect("Failed to read input");
        let yearsofexp: i32 = yearsinput.trim().parse().expect("Failed to read input");
        if yearsofexp >=8  &&yearsofexp <= 10 {
            println!("valid: office manager has an EL1 level of 8-10");
        }
        else{ 
            println!("invalid: an office manager should have 8-10 years of experience");
        }
      }
      else if index == 4 {
         println!("how many years have you serve");
            let mut yearsinput = String::new();
        std::io::stdin().read_line(&mut yearsinput).expect("Failed to read input");
        let yearsofexp: i32 = yearsinput.trim().parse().expect("Failed to read input");
        if yearsofexp >=11 &&yearsofexp <= 13  {
            println!("valid: director has an EL2 level of 11-13");
        }
        else{ 
            println!("invalid: A director should have 11-13 years of experience");
        }
      }
      else if index == 5 {
         println!("how many years have you serve");
            let mut yearsinput = String::new();
        std::io::stdin().read_line(&mut yearsinput).expect("Failed to read input");
        let yearsofexp: i32 = yearsinput.trim().parse().expect("Failed to read input");
        if yearsofexp > 13 {
            println!("valid: CEO has an SES level greater than 13 years");
        }
        else{ 
            println!("invalid: CEO should have 13 years of experience");
        }        let index:usize = inputyrs.trim().parse().expect("failed to read input");
        if index > 5 {
            println!("Invalid category, enter a number from 1 to 5");
            return;
        }
        let chosen_years: String = years[index].to_string();
     }
   }          
    }
    if index == 1{
        println!("Academic chosen \n Choose your category using index \n 0 not given\n 1 for Research assistant \n 2 for PHD candidate\n 3 for post Doc researcher\n 4 for Senior researcher \n 5 for Dean");

        let years = vec!["-","Research assistant","PHD candidate","post Doc researcher","Senior researcher","Dean"];
        let mut inputyrs = String::new();
        println!("Enter an index of 1-5");
        std::io::stdin().read_line(&mut inputyrs).expect("failed to read input");
        let index:usize = inputyrs.trim().parse().expect("failed to read input");
        let chosen_years: String = years[index].to_string();
        if index == 0 {
            println!("theres no category retry");
        }
         if index == 1{ println!("how many years have you serve");
            let mut yearsinput = String::new();
        std::io::stdin().read_line(&mut yearsinput).expect("Failed to read input");
        let yearsofexp: i32 = yearsinput.trim().parse().expect("Failed to read input");
        if yearsofexp >= 3 &&yearsofexp <= 4 {
            println!("valid: Research assistant has an Aps level of 3-4");
        }
        else{ println!("invalid: an research assistant should have 3-4 years of experience")
            }

      }
      else if index == 2 {
         println!("how many years have you serve");
            let mut yearsinput = String::new();
        std::io::stdin().read_line(&mut yearsinput).expect("Failed to read input");
        let yearsofexp: i32 = yearsinput.trim().parse().expect("Failed to read input");
        if yearsofexp >= 5 &&yearsofexp <= 7 {
            println!("valid: Phd candidate has an Aps level of 5-7 ");
        }
        else{ 
            println!("invalid: an phd candidate should have 5-7 years of experience");
        }
      }
      else if index == 3 {
         println!("how many years have you serve");
            let mut yearsinput = String::new();
        std::io::stdin().read_line(&mut yearsinput).expect("Failed to read input");
        let yearsofexp: i32 = yearsinput.trim().parse().expect("Failed to read input");
        if yearsofexp >=8  &&yearsofexp <= 10 {
            println!("valid: Doc researcher has an EL1 level of 8-10");
        }
        else{ 
            println!("invalid: an Doc researcher should have 8-10 years of experience");
        }
      }
      else if index == 4 {
         println!("how many years have you serve");
            let mut yearsinput = String::new();
        std::io::stdin().read_line(&mut yearsinput).expect("Failed to read input");
        let yearsofexp: i32 = yearsinput.trim().parse().expect("Failed to read input");
        if yearsofexp >=11 &&yearsofexp <= 13  {
            println!("valid: Senior researcher has an EL2 level of 11-13");
        }
        else{ 
            println!("invalid: an Senior researcher should have 11-13 years of experience");
        }
      }
      else if index == 5 {
         println!("how many years have you serve");
            let mut yearsinput = String::new();
        std::io::stdin().read_line(&mut yearsinput).expect("Failed to read input");
        let yearsofexp: i32 = yearsinput.trim().parse().expect("Failed to read input");
                let index:usize = inputyrs.trim().parse().expect("failed to read input");
        if index > 5 {
            println!("Invalid category, enter a number from 0 to 5");
            return;
        }
        let chosen_years: String = years[index].to_string();
        if yearsofexp > 13 {
            println!("valid: Dean has an SES level greater than 13 years");
        }
        else{ 
            println!("invalid: Dean should have 13 years of experience");
        }
    }
   }
   if index == 2 {
        println!("lawyer chosen \n Choose your category using index \n 0 for Paralegal\n 1 for Junior Associate \n 2 for Associate\n 3 for Senior associate 1-2\n 4 Senior Associate 3-4 \n 5 for Partner");

        let years = vec!["Paralegal","Junior associate","Associate","Senior associate 1-2","Senior associate 3-4","Partner"];
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
            println!("valid: Paralegal has an Aps level of 1-2");
        }
        else{ 
            println!("invalid: an Paralegal should have 1 to 2 years of experience");
            }
        
      }
       else if index == 1{ println!("how many years have you serve");
            let mut yearsinput = String::new();
        std::io::stdin().read_line(&mut yearsinput).expect("Failed to read input");
        let yearsofexp: i32 = yearsinput.trim().parse().expect("Failed to read input");
        if yearsofexp >= 3 &&yearsofexp <= 4 {
            println!("valid: Junior associate has an Aps level of 3-4");
        }
        else{ println!("invalid: an Junior associate should have 3-4 years of experience")
            }

      }
      else if index == 2 {
         println!("how many years have you serve");
            let mut yearsinput = String::new();
        std::io::stdin().read_line(&mut yearsinput).expect("Failed to read input");
        let yearsofexp: i32 = yearsinput.trim().parse().expect("Failed to read input");
        if yearsofexp >= 5 &&yearsofexp <= 7 {
            println!("valid: associate has an Aps level of 5-7 ");
        }
        else{ 
            println!("invalid: an Associate should have 5-7 years of experience");
        }
      }
      else if index == 3 {
         println!("how many years have you serve");
            let mut yearsinput = String::new();
        std::io::stdin().read_line(&mut yearsinput).expect("Failed to read input");
        let yearsofexp: i32 = yearsinput.trim().parse().expect("Failed to read input");
        if yearsofexp >=8  &&yearsofexp <= 10 {
            println!("valid: Senior associate  has an EL1 level of 8-10");
        }
        else{ 
            println!("invalid: an Senior Associate should have 8-10 years of experience");
        }
      }
      else if index == 4 {
         println!("how many years have you serve");
            let mut yearsinput = String::new();
        std::io::stdin().read_line(&mut yearsinput).expect("Failed to read input");
        let yearsofexp: i32 = yearsinput.trim().parse().expect("Failed to read input");
        if yearsofexp >=11 &&yearsofexp <= 13  {
            println!("valid: Senior Associate 3-4 has an EL2 level of 11-13");
        }
        else{ 
            println!("invalid: an Senior Associate 3-4 should have 11-13 years of experience");
        }
      }
      else if index == 5 {
         println!("how many years have you serve");
            let mut yearsinput = String::new();
        std::io::stdin().read_line(&mut yearsinput).expect("Failed to read input");
        let yearsofexp: i32 = yearsinput.trim().parse().expect("Failed to read input");
                let index:usize = inputyrs.trim().parse().expect("failed to read input");
        if index > 5 {
            println!("Invalid category, enter a number from 0 to 5");
            return;
        }
        let chosen_years: String = years[index].to_string();
        if yearsofexp > 13 {
            println!("valid: Partner has an SES level greater than 13 years");
        }
        else{ 
            println!("invalid: Partner should have 13 years of experience");
        }
        }  
    }
    if index == 3 {
        println!("Teacher chosen \n Choose your category using index \n 0 for Placement\n 1 for Classroom teacher \n 2 for Senior teacher\n 3 for leading teacher\n 4 for deputy principal \n 5 for Principal");

        let years = vec!["Placement","Classroom teacher","Senior teacher","Leading teacher","Deputy principal","Principal"];
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
            println!("valid: Placement has an Aps level of 1-2");
        }
        else{ 
            println!("invalid: an Placement should have 1 to 2 years of experience");
            }
        
      }
       else if index == 1{ println!("how many years have you serve");
            let mut yearsinput = String::new();
        std::io::stdin().read_line(&mut yearsinput).expect("Failed to read input");
        let yearsofexp: i32 = yearsinput.trim().parse().expect("Failed to read input");
        if yearsofexp >= 3 &&yearsofexp <= 4 {
            println!("valid: Classroom teacher has an Aps level of 3-4");
        }
        else{ println!("invalid: an Classroom teacher should have 3-4 years of experience")
            }

      }
      else if index == 2 {
         println!("how many years have you serve");
            let mut yearsinput = String::new();
        std::io::stdin().read_line(&mut yearsinput).expect("Failed to read input");
        let yearsofexp: i32 = yearsinput.trim().parse().expect("Failed to read input");
        if yearsofexp >= 5 &&yearsofexp <= 7 {
            println!("valid: Senior Teacher has an Aps level of 5-7 ");
        }
        else{ 
            println!("invalid: an Senior Teacher should have 5-7 years of experience");
        }
      }
      else if index == 3 {
         println!("how many years have you serve");
            let mut yearsinput = String::new();
        std::io::stdin().read_line(&mut yearsinput).expect("Failed to read input");
        let yearsofexp: i32 = yearsinput.trim().parse().expect("Failed to read input");
        if yearsofexp >=8  &&yearsofexp <= 10 {
            println!("valid: Leading teacher has an EL1 level of 8-10");
        }
        else{ 
            println!("invalid: an leading teacher should have 8-10 years of experience");
        }
      }
      else if index == 4 {
         println!("how many years have you serve");
            let mut yearsinput = String::new();
        std::io::stdin().read_line(&mut yearsinput).expect("Failed to read input");
        let yearsofexp: i32 = yearsinput.trim().parse().expect("Failed to read input");
        if yearsofexp >=11 &&yearsofexp <= 13  {
            println!("valid: Deputy principal has an EL2 level of 11-13");
        }
        else{ 
            println!("invalid: an Deputy principal should have 11-13 years of experience");
        }
      }
      else if index == 5 {
         println!("how many years have you serve");
            let mut yearsinput = String::new();
        std::io::stdin().read_line(&mut yearsinput).expect("Failed to read input");
        let yearsofexp: i32 = yearsinput.trim().parse().expect("Failed to read input");
        if yearsofexp > 13 {
            println!("valid: Principal has an SES level greater than 13 years");
        }
        else{ 
            println!("invalid: principal should have 13 years of experience");
        }
    }  
  }                        
}  
