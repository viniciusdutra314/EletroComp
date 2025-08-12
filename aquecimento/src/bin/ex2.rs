use num_bigint::{BigInt, ToBigInt};

fn fibonacci(n:usize) -> BigInt{
	let mut f0=BigInt::ZERO;
	let mut f1=1.to_bigint().unwrap();
	for _ in 0..n {
		let f_next=f0+&f1;
		f0=f1;
		f1=f_next;
	}
	return f0;
}


fn main(){
	let mut i=2;
	loop {
	    println!("{}",fibonacci(i+1)/fibonacci(i));
		i+=1;
	}
}