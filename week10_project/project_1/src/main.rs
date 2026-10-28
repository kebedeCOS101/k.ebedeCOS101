struct Branding{
    brand:String,
    quantity:u32,
    price:u32
}
fn main() {
    let brand1 = Branding{
        brand:String::from("Hp"),
        quantity:10,
        price:650_000,
    };
    let brand2 = Branding{
        brand:String::from("IBM"),
        quantity : 6,
        price : 755_000,
    };
    let brand3 = Branding{
        brand:String::from("Toshiba"),
        quantity : 10,
        price : 550_000,
    };
    let brand4 = Branding{
        brand:String::from("Dell"),
        quantity : 4,
        price : 850_000,
    };
    impl Branding {
        fn total(&self)->u32{
            3*self.price
        }
    }
    println!("This total of 3 laptop of each brand is \n{}",brand1.total()+brand2.total()+brand3.total()+brand4.total());
}    
    
     
