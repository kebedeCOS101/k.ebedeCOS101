use std::io::Read;
fn main() {
    let mut file = std::fs::File::open("staff_tb.sql");
    let mut contents = String::new();
    file.expect("failed  to read files").read_to_string(&mut contents).unwrap();
    print!("{}",contents)
}
