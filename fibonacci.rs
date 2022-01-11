fn fibo(num: u64) -> u64 {
	let mut first = 0;
	let mut second = 1;
	
	if num == 0 {
		first
	} else if num == 1 {
		second
	} else {
		let mut counter = 2;

		loop {
			let third = first + second;

			if counter == num {
				break third
			} else {
				first = second;
				second = third;
				counter += 1;
			}
		}
	}
}

fn main() {
	const N: u64 = 50;

	println!("{} of fibonacci is: {}", N, fibo(N));
}