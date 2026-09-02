fn main(){
	let p:f64 = 520_000_000.0;
	let r:f64 = 10.0;
	let n:f64 = 5.0;
	
	
    let a:f64 = p * (1.0 + (r/100.0)).powf(n);
	
	let c = a - p; // c = compound interest

	println!("amount is {}", a);
	println!("Compound interest = {}", c);
}