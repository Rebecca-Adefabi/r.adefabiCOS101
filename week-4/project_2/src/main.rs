//incentive calculator
use std::io;

fn main() {
    loop{

        let mut input1 = String::new();
        let mut input2 = String::new();
        let mut input3 = String::new();
        
        println!("\nThe incentive calculator");
        println!("\nWhat will you like to do;");
        println!("\nTo calculate your incentive, enter 1");

        println!("To exit, enter 2:");
        io::stdin().read_line(&mut input1).expect("Enter valid digit");
        let ca:u8 = input1.trim().parse().expect("Enter valid digit");

        if ca == 1{
            println!("How old are you: ");
            io::stdin().read_line(&mut input2).expect("Enter valid digit");
            let age:u8 = input2.trim().parse().expect("Enter valid digit");
            

            println!("are you experienced? true/false");
            io::stdin().read_line(&mut input3).expect("Enter valid digit");
            let exp:bool = input3.trim().parse().expect("Enter valid digit");

            if age >= 20 && exp ==false{
                println!("\nYour annual incentive is 100_000 naira");
            }      
            else if age >= 40 && exp == true{
                println!("\nYour annual incentive is 1_560_000 naira");
            }
            else if age >=30 && exp ==true{
                println!("\nYour annual incentive is 1_480_000 naira");
            }     
            else if age >=18 && exp ==true{
                println!("\nYour annual incentive is 1_300_000 naira");
            }
            else{
                println!("\nYou are not qualified to be an employee");
            }

        }
        else if ca == 2{
            println!("Thank you for using our services");
            break;
        }
        else{
            println!("Error:Please, enter either 1 or 2");
            continue;
        }

    }
    

}
