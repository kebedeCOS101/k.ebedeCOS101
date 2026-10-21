use std::io::Write;

 fn main() {
    let header = "lager,stout,non-alcoholic\n";//declaring Header 
    let row2 = "33 Export,Legaend,Maltina\n";//declaring rows
    let row3 = "Goldberg,Williams,Malta gold\n";
    let row4 = "Gulder,,Fayrouz\n";
    let row5 = "heineken,,,\n";
    let row6 = "star,,,\n";
    let mut file = std::fs::File::create("data.csv").expect("Failed to create");
    file.write_all(header.as_bytes()).expect("Write failed");//writing files
    file.write_all(row2.as_bytes()).expect("Write failed");
    file.write_all(row3.as_bytes()).expect("Write failed");
    file.write_all(row4.as_bytes()).expect("Write failed");
    file.write_all(row5.as_bytes()).expect("Write failed");
    file.write_all(row6.as_bytes()).expect("Write failed");



}
