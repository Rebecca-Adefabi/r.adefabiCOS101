fn main(){
	let p:f64 = 1000.0;
	let r:f64 = 1.0;
	let t:f64 = 2.0;

	// simple interest
	let si = p * (r/100.0) * t;
    
    print!("Simple interest = {}", si);	
}