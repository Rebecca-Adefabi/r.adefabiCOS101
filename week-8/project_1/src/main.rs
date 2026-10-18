use std::io;

fn input_num() -> u8 {
    loop {
        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Failed to read input");

        match input.trim().parse::<u8>() {
            Ok(num) => return num,
            Err(_) => {
                println!("Please enter a valid number:");
            }
        }
    }
}
fn input_str() -> String {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}
fn get_index(years: u8, field: u8) -> usize {

    // Handle overlap years
    if years == 5 || years == 8 || years == 10 || years == 13 {

        println!("This year falls between two APS levels.");
        println!("Enter your current role:");

        let role = input_str();

        match field {

            // Office Admin
            1 => {
                if years == 5 {
                    if role.to_lowercase() == "administrator" {
                        return 1;
                    } else {
                        return 2;
                    }
                }

                if years == 8 {
                    if role.to_lowercase() == "senior administrator" {
                        return 2;
                    } else {
                        return 3;
                    }
                }

                if years == 10 {
                    if role .to_lowercase() =="office manager"{
                        return 3;
                    } 
                    else {
                        return 4;
                    }
                }

                if years == 13 {
                    if role .to_lowercase() =="director" {
                        return 4;
                    } 
                    else {
                        return 5;
                    }
                }
            }

            // Academic
            2 => {
                if years == 5 {
                    if role .to_lowercase() == "research assistant" {
                        return 1;
                    } 
                    else {
                        return 2;
                    }
                }

                if years == 8 {
                    if role.to_lowercase() == "phd candidate" {
                        return 2;
                    } 
                    else {
                        return 3;
                    }
                }

                if years == 10 {
                    if role.to_lowercase() == "post-doc researcher" {
                        return 3;
                    } 
                    else {
                        return 4;
                    }
                }

                if years == 13 {
                    if role.to_lowercase() == "senior lecturer" {
                        return 4;
                    } 
                    else {
                        return 5;
                    }
                }
            }

            // Lawyer
            3 => {
                if years == 5 {
                    if role.to_lowercase() =="junior associate" {
                        return 1;
                    } 
                    else {
                        return 2;
                    }
                }

                if years == 8 {
                    if role.to_lowercase() == "associate" {
                        return 2;
                    } 
                    else {
                        return 3;
                    }
                }

                if years == 10 {
                    if role.to_lowercase() == "senior associate 1-2" {
                        return 3;
                    } 
                    else {
                        return 4;
                    }
                }

                if years == 13 {
                    if role.to_lowercase() == "senior associate 3-4" {
                        return 4;
                    } 
                    else {
                        return 5;
                    }
                }
            }

            // Teacher
            4 => {
                if years == 5 {
                    if role.to_lowercase()== "classroom teacher"{
                        return 1;
                    } 
                    else {
                        return 2;
                    }
                }

                if years == 8 {
                    if role.to_lowercase() =="senior teacher"{
                        return 2;
                    } 
                    else {
                        return 3;
                    }
                }

                if years == 10 {
                    if role.to_lowercase() == "leading teacher" {
                        return 3;
                    } 
                    else {
                        return 4;
                    }
                }

                if years == 13 {
                    if role.to_lowercase() == "deputy principal" {
                        return 4;
                    } 
                    else {
                        return 5;
                    }
                }
            }

            _ => {
                println!("Enter valid input");
            }
        }
    }

    // Non-overlap years
    if years >= 1 && years <= 2 {
        0
    } else if years >= 3 && years <= 4 {
        1
    } else if years >= 6 && years <= 7 {
        2
    } else if years == 9 {
        3
    } else if years >= 11 && years <= 12 {
        4
    } else {
        5
    }
}
fn main() {
    let office_admin = vec![
        ("APS 1-2", "Intern"),
        ("APS 3-5", "Administrator"),
        ("APS 5-8", "Senior Administrator"),
        ("EL1 8-10", "Office Manager"),
        ("EL2 10-13", "Director"),
        ("SES", "CEO"),
    ];

    let academic = vec![
        ("APS 1-2", "Not Applicable"),
        ("APS 3-5", "Research Assistant"),
        ("APS 5-8", "PhD Candidate"),
        ("EL1 8-10", "Post-Doc Researcher"),
        ("EL2 10-13", "Senior Lecturer"),
        ("SES", "Dean"),
    ];

    let lawyer = vec![
        ("APS 1-2", "Paralegal"),
        ("APS 3-5", "Junior Associate"),
        ("APS 5-8", "Associate"),
        ("EL1 8-10", "Senior Associate 1-2"),
        ("EL2 10-13", "Senior Associate 3-4"),
        ("SES", "Partner"),
    ];

    let teacher = vec![
        ("APS 1-2", "Placement"),
        ("APS 3-5", "Classroom Teacher"),
        ("APS 5-8", "Senior Teacher"),
        ("EL1 8-10", "Leading Teacher"),
        ("EL2 10-13", "Deputy Principal"),
        ("SES", "Principal"),
    ];

    println!("Select your field:");
    println!("1. Office Admin");
    println!("2. Academic");
    println!("3. Lawyer");
    println!("4. Teacher");

    let field = input_num();

    println!("Enter years of experience:");
    let years = input_num();

    let index = get_index(years, field);

    match field {
        1 => {
            println!("\nAPS Level: {}", office_admin[index].0);
            println!("Position: {}", office_admin[index].1);
        }

        2 => {
            println!("\nAPS Level: {}", academic[index].0);
            println!("Position: {}", academic[index].1);
        }

        3 => {
            println!("\nAPS Level: {}", lawyer[index].0);
            println!("Position: {}", lawyer[index].1);
        }

        4 => {
            println!("\nAPS Level: {}", teacher[index].0);
            println!("Position: {}", teacher[index].1);
        }

        _ => {
            println!("Invalid field selected.");
        }
    }
}