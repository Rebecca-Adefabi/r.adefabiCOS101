


fn input() -> String{
    let mut inp = String::new();
    std::io::stdin().read_line(&mut inp).expect("Failed to read");
    inp.trim().to_string()
    
}
use std::io::Write;

    
fn main() {
    println!("PAU Student Information Management System");

    users_data();
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
    let mut file = OpenOptions::new().append(true).create(true).open("sims.csv").expect("cannot open");
    file.write_all(sims_new.as_bytes()).expect("Done"); 
    println!("Data saved to file");

}

