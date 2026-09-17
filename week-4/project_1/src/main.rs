//To calculate quadratic roots

use std::io;

fn main() {
    loop{
        println!("Calculate quadratic roots");
        let mut input1 = String::new();
        let mut input2 = String::new();
        let mut input3 = String::new();
        println!("enter ctrl + c to quit at any time");

        
        println!("enter the value of a:");
        io::stdin().read_line(&mut input1).expect("enter a valid digit");
        let a:f32 = input1.trim().parse().expect("enter a valid digit");
      

         
        //else{continue;}
        
        println!("enter the value of b");
        io::stdin().read_line(&mut input2).expect("enter a valid digit");
        let b:f32 = input2.trim().parse().expect("enter a valid digit");

        println!("enter the value of c");
        io::stdin().read_line(&mut input3).expect("enter a valid digit");
        let c:f32 = input3.trim().parse().expect("enter a valid digit");
        
        if a <= 0.0{println!("this is a linear equation not quadratic");
        continue;
         }

        
        let d:f32 = b*b - 4.0*a*c;
        println!(" d = {}", d);

        if d > 0.0{
            println!("Two distinct roots");
            let x1:f32 = (-b + d.sqrt())/2.0*a;
            let x2:f32 = (-b - d.sqrt())/2.0*a;
            println!(" x = {} or x = {} ",x1, x2); 

        }
        else if d == 0.0{
            println!("exactly one real root");
            let x:f32 = -b/(2.0*a);
            println!("x = {} ",x);
        }
         else{
            println!("No real roots");
            let xr:f32 = -b/(2.0*a);
            let xi:f32 = (-d).sqrt()/(2.0 * a);
            println!("the roots are imaginary: {:.2} + {:.2}i and {:.2} - {:.2}i ", xr, xi, xr ,xi);

         }
    }

}
