use std::io;

fn input_num() -> u8 {
    loop {
        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Failed to read input");

        match input.trim().parse::<u8>() {
            Ok(num) => return num,
            Err(_) => println!("Please enter a valid number:"),
        }
    }
}

fn input_str() -> String {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read input");

    input.trim().to_string()
}

fn main() {
    let mut candidates: Vec<(String, u8)> = Vec::new();

    println!("How many candidates were interviewed?");
    let num_candidates = input_num();

    for i in 1..=num_candidates {
        println!("\nEnter Candidate {} Name:", i);
        let name = input_str();

        println!("Enter Candidate {} Years of Experience:", i);
        let years = input_num();

        candidates.push((name, years));
    }

    let mut most_experienced = &candidates[0];

    for candidate in &candidates {
        if candidate.1 > most_experienced.1 {
            most_experienced = candidate;
        }
    }

    println!("\nRESULT");
    println!("Most Experienced Candidate: {}", most_experienced.0);
    println!(
        "Years of Programming Experience: {}",
        most_experienced.1
    );
}