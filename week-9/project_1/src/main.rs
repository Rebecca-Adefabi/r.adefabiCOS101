use std::io::Write;

fn main(){

    let announce = "Nigerian breweries\n";
    let product = "Lager,Stout,Non-alcoholic
33 Export,Legend,Maltina
Desperados,Turbo King,Amstel Malta
Goldberg,Williams,Malta Gold
Gulder,,Fayrouz 
Heineken,, 
Star,,";
    

    let mut file = std::fs::File::create("Nigerianbreweries.csv").expect("failed to create");
    file.write_all(announce.as_bytes()).expect("Write failed");
    file.write_all(product.as_bytes()).expect("failed to read");
    println!("Data written to file.");
}
