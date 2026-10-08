use std::io;
fn input() -> u8{ //making a funcion for input reading
    loop{
        let mut input= String::new();
            io::stdin().read_line(&mut input).expect("enter valid input");
            let value:u8 =match input.trim().parse(){
                Ok(num)=> num, 
                Err(_)=> {
                    println!("Enter valid number");
                    continue;
                }
                
            };
        return value;
    }   
}

fn main() {
    loop{
        println!("Shapes calculator");
        
        let shapes :[&str; 5] = ["Trapezium","Rhombus","Parallelogram", "cube","Cylinder"];
        for index in 0..5 {
            println!("Enter {} for shape {}", index, shapes[index]);
        }
        println!("To exit enter 5");
        println!("What shape do you want to calculate? ");
        let choice =  input();
        if choice == 0{
            trapezium()
        } 
        else if choice == 1{
            rhombus()
        }
        else if choice == 2{
            parallelogram()
        }
        else if choice == 3{
            cube()
        }
        else if choice == 4{
            cylinder()
        }
        else if choice == 5{
            break;
        }
        else{
            println!("Folow instructions, Enter digits from 0-5");
        }


    }
    
}
fn trapezium(){
    println!("To calculate the area of a trapezium");
    println!("Enter the value of height:");
    let h = input()as f32;
    println!("enter the value of first base");
    let b1 = input()as f32;
    println!("Enter the value of the second base");
    let b2 = input()as f32;
    let total:f32 = (h/2.0)*(b1+b2);
    println!("The area of trapezium = {:.3}",total);

}
fn rhombus(){
    println!("To calculate the area of Rhombus");
    println!("Enter the value of diagonal1:");
    let d1 = input()as f32;
    println!("Enter the value of diagonal2:");
    let d2 = input()as f32;
    let total:f32 = 0.5*(d1*d2);
    println!("The area of the Rhombus = {:.3}",total);
}
fn parallelogram(){
    println!("To calculate the area of Trapezium");
    println!("Enter the value of base:");
    let b = input()as f32;
    println!("Enter the value of altitude");
    let a = input()as f32;
    let total:f32 = b*a;
    println!("The area of the parallelogram = {:.}", total);
}
fn cube(){
    println!("To calculate the surface area of a Cube");
    println!("Enter the value of side:");
    let s = input()as f32;
    let total:f32 = 6.0*s*s;
    println!("the surface area of the cube = {:.}", total);
}
fn cylinder(){
    println!("To calculate the volume of a cylinder");
    println!("Enter the value of radius:");
    let r = input()as f32;
    println!("Enter the value of height:");
    let h = input()as f32;
    let total:f32 = (22.0/7.0)*r*r*h; 
    println!("The volume of the cylinder = {:.}", total);
}
