use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    
    let student = vec![
        ("Student name","Matric number","Department","Level"),
        ("Oluchi Mordi", "ACC10211111","Accounting","300"),
        ("Adams Aliyu", "ECO10110101","Economics","100"),
        ("Shania Bolade", "CSC10328828","Compuer","200"),
        ("Adekunle Gold", "EEE11020202","Electrical","200"),
        ("Bianca Edemoh","MEE10202001", "mechaical","100"),
    ];
    let mut sims = String::new();
    for index in 0..6{
        println!("{:?},{:?},{:?},{:?}",student[index].0,student[index].1,student[index].2,student[index].3);
    }    
    for index in 0..6{
        sims.push_str ( 
            &format!("{},{},{},{}\n",student[index].0,student[index].1,student[index].2,student[index].3)
            )
    
    };
    let mut file = std::fs::File::create("sims.csv").expect("Create failed");
    file.write_all("PAU Students Information Management Portal\n"
        .as_bytes()).expect("Write failed");
    file = OpenOptions::new().append(true).open("sims.csv").expect("cannot open");
    file.write_all(sims.as_bytes()).expect("Done");
    println!("Success");



                 
}

