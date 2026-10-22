use std::fs;

fn main() {
    fs::remove_file("data.txt").expect("could not remove file");
    printlln!("file is removed");
    
}
