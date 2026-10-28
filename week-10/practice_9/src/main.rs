// define dimensions of a rectangle
struct rectangle{
    width:u32, height:u32
}
//logic to calculate area of a rectangle
impl rectangle{
    fn area(&self)->u32{
        //use the . operator to fetch the value of a field via the self keyboard
        self.width * self.height
    }
}
fn main() {
    // instanatiate the structure
    let small = rectangle{
      width:10,
      height:20,
   };
   //print the rectangle's area
   println!("width is {} \n height is {} \n area or rectangle
    is {}",small.width,small.height,small.area());
}
