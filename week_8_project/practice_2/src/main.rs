use std::io;

fn main() {
    println!("Welcome to EY Interviews");
    println!("How many applicants would you like to interview?");
    let mut count_input = String::new();
    io::stdin().read_line(&mut count_input).expect("Failed to read input");
    let count: usize = count_input.trim().parse().expect("Invalid input");

    // nobody to compare, so stop here
    if count == 0 {
        println!("No applicants were entered");
        return;
    }

    // each applicant is (name, years)
    let mut applicants: Vec<(String, u8)> = Vec::new();

    for n in 0..count {
        println!("Applicant {}", n + 1);

        let mut name_input = String::new();
        println!("What is your name?");
        io::stdin().read_line(&mut name_input).expect("Failed to read input");
        let applicant_name = name_input.trim().to_string();

        let mut years_input = String::new();
        println!("How many years of experience do you have?");
        io::stdin().read_line(&mut years_input).expect("Failed to read input");
        let applicant_years: u8 = years_input.trim().parse().expect("Invalid input");

        applicants.push((applicant_name, applicant_years));
    }

    // assume the first person is the best, then check everyone else
    let mut best: usize = 0;

    for index in 1..applicants.len() {
        // > means the first one entered wins a tie
        if applicants[index].1 > applicants[best].1 {
            best = index;
        }
    }

    println!("The most experienced applicant is {} with {} years",
             applicants[best].0, applicants[best].1);
}
