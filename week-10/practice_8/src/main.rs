struct Employee{
    ceo:String,
    company:String,
    age:u32
}
fn main() {
    // initialize a structure 
    let emp1= Employee{
        company:String::from("Microsoft Corporation"),
        ceo:String::from("Satya Nadella"),
        age:56
    };
    let emp2 = Employee{
        company:String::from("Google inc,"),
        ceo:String::from("Sundai pichai"),
        age:51
    };
    //pass emp1 and emp2 to display()
    display(emp1);
    display(emp2);
}
//fetch values of specific structure fields using the 
// operator and print it to the consolte
fn display(emp:Employee){
    println!("name is :{} company is {} age is 
        {}",emp.ceo,emp.company,emp.age);

}
