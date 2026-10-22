use std::io::Read;

fn main(){
    let mut file = std::fs::File::open("Nigerian_breweries.csv").expect("failed to open");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("failed to read");
    print!("{}", contents);
}