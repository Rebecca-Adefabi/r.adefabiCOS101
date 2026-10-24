


fn input() -> String{
    let mut inp = String::new();
    std::io::stdin().read_line(&mut inp).expect("Failed to read");
    inp.trim().to_string()
    
}
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
        println!("{} {} {} {}",student[index].0,student[index].1,student[index].2,student[index].3);
    }    
    for index in 0..6{
        sims.push_str ( 
            &format!("{},{},{},{}\n",student[index].0,student[index].1,student[index].2,student[index].3)
            )
    
    };
    let mut file = std::fs::File::create("sims.csv").expect("Create failed");
    file.write_all("PAU Students Information Management Portal\n"
        .as_bytes()).expect("Write failed");
    file.write_all(sims.as_bytes()).expect("couldn't");
    
    println!("Success, do you want to add more records?");
    println!("Enter yes or no");
    let resp = input().trim().to_lowercase();
    if resp == "yes"{
        users_data()
    }
    else if resp == "no"{
        println!("Thanks for using this service");
    }
    else{
        println!("wrong option");
    }



                 
}
use std::fs::OpenOptions;
fn users_data(){//ffunction to collect more data from the user
    println!("How many student data do you want to enter?");
    let no_of_students:u32 = input().trim().parse().expect("invalid");
    let mut sims_new = String::new();
    for i in 0..no_of_students{
        println!("Enter student{} name",i+1);
        let student_name = input();
        println!("Enter student{} matric number",i+1);
        let student_matric = input();
        println!("Enter student{} department",i+1);
        let student_department = input();
        println!("Enter stuent{} level",i+1);
        let student_level = input();
         sims_new.push_str(
            &format!("{},{},{},{}\n",student_name,student_matric,student_department,student_level)
            )


       
    }
    let mut file = OpenOptions::new().append(true).open("sims.csv").expect("cannot open");
    file.write_all(sims_new.as_bytes()).expect("Done"); 
    println!("Data saved to file");

}

