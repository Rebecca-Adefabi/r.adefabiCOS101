
struct Laptop {
    brand: String,
    price: u32,
    quantity:u32,
    

}
impl Laptop {
    fn purchase(&self)->u32{
        self.price * self.quantity
        //println!("the total price for {} purchased is {}naira",laptop.brand,t;

    }
    
}
fn main() {
    let laptop1 = Laptop{
        brand:String::from("HP"), price:650_000,quantity:3
    };
    let laptop2 = Laptop{
        brand:String::from("IBM"), price:750_000,quantity:3
    };
    let laptop3 =Laptop{
        brand:String::from("Toshiba"), price:550_000, quantity:3
    };
    let laptop4 = Laptop{
        brand:String::from("Dell"), price:850_000, quantity:3
    };
    let purchase1 = laptop1.purchase();
    let purchase2 = laptop2.purchase();
    let purchase3 = laptop3.purchase();
    let purchase4 = laptop4.purchase();

    println!("Cost for 3 {} laptop is {}\n", laptop1.brand, purchase1);
    println!("Cost for 3 {} laptop is {}\n", laptop2.brand, purchase2);
    println!("Cost for 3 {} laptop is {}\n", laptop3.brand, purchase3);
    println!("Cost for 3 {} laptop is {}\n", laptop4.brand, purchase4);

    let total_purchase = purchase1 + purchase2 + purchase3 + purchase4;
    println!("Total purchase is {}",total_purchase);


}
