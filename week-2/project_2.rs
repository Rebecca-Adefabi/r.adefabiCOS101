fn main(){
	let p_0:f64 = 450_000.00; 
	let p_1:f64 = 1_500_000.0; 
	let p_2:f64 = 750_000.0; 
	let p_3:f64 = 2_850_000.0; 
	let p_4:f64 = 250_000.0;
	let q_0:f64 = 2.0;
	let q_1:f64 = 1.0;
	let q_2:f64 = 3.0;
	let q_3:f64 = 3.0;
	let q_4:f64 = 1.0;

	let _n:f64 = q_0 + q_1 + q_2 + q_3 + q_4;
	let n:f64 = 5.0;
	let s:f64 = (p_0*q_0) + (p_1*q_1) + (p_2*q_2) + (p_3*q_3) + (p_4*q_4);
	let a:f64 = s/n;

	println!("Sum = {}", s);
	println!("Average = {}", a); 
}