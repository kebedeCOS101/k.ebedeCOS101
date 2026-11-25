use std::io::Read;
fn main() {
    let mut file = std::fs::File::open("cos101_db.sql");
    let mut contents = String::new();
    file.expect("Failed to open file").read_to_string(&mut contents).unwrap();
    print!("{}",contents);
}
