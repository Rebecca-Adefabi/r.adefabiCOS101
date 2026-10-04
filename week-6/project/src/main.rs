use std::io;

fn main() {
    loop{
        println!("\nWhat will you like to do?");
        println!("Enter 1 to order a meal, enter 2 to exit");
        let mut input1 = String::new();
        io::stdin().read_line(&mut input1).expect("enter valid number");
        
        let choice:u8 = match input1.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number.");
                continue;
            }
        };
        
        let p = ("P", "Poundo Yam / Edinkaiko Soup", 3200);
        let f = ("F", "Fried Rice & Chicken", 3000);
        let a = ("A", "Amala & Ewedu Soup", 2500);
        let e = ("E", "Eba & Egusi Soup", 2000);
        let w = ("W", "White Rice & Stew", 2500);
        

        if choice == 1{

        

            println!("Welcome to PAU Restaurant, what will you like to have");
            
            println!("{} {} N{}", p.0, p.1, p.2);
            println!("{} {} N{}", f.0, f.1, f.2);
            println!("{} {} N{}", a.0, a.1, a.2);
            println!("{} {} N{}", e.0, e.1, e.2);
            println!("{} {} N{}", w.0, w.1, w.2);

            println!("\n enter the letter of your choice");
            let mut input2 = String::new();
            io::stdin().read_line(&mut input2 ).expect("enter valid letter");
            let food_choice = input2.trim().to_uppercase();

            let (food_name, price) = match food_choice.as_str() {
            "P" => ("Poundo Yam / Edinkaiko Soup", 3200.0),
            "F" => ("Fried Rice & Chicken", 3000.0),
            "A" => ("Amala & Ewedu Soup", 2500.0),
            "E" => ("Eba & Egusi Soup", 2000.0),
            "W" => ("White Rice & Stew", 2500.0),
            _ => {
                println!("Invalid food choice");
                continue;
            }
        };


           
            println!("How many portions:");
            let mut input3 = String::new();
            io::stdin().read_line(&mut input3).expect("enter valid number");
            let quan:f32 = match input3.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Quantity must be a number.");
                    continue;
                }
            };

            println!("you ordered {} portions of {}", quan, food_name);
            let mut total = price * quan;
            if total >0.0 && total < 10000.0{
                println!("Total = N{}", total);
            }    
            else if total >= 10000.00{
                total = total - (total * 0.05);
                println!("You earned a 5% discount for over N10,000 purchase");
                println!("you are to pay {}", total);
            }


        }
        else if choice == 2{
            println!("Enjoy your meal");
            break;
        }
        else{
            println!("enter 1 or 2");
        }
    }

}    
    
