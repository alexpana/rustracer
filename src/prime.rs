#![allow(dead_code)]

fn sqrt(number: i32) -> i32 {
	(number as f32).sqrt() as i32
}

fn is_prime(number : i32) -> bool {
	use std::ops::*;

	if number < 2 {
		return false;
	}

	if number == 2 {
		return true;
	}

	let custom_range = Range{start: 2, end: sqrt(number) + 1};

	for i in custom_range {
		if number % i == 0 {
			return false;
		}
	}

	true
}

fn print_primality(number : i32) {
	let prime = is_prime(number);
	if prime {
		println!("{} is prime", number);
	} else {
		println!("{} is not prime", number);
	}
}

fn main() {
}

#[test]
fn test_primality() {
	assert_eq!(is_prime(0), false);
	assert_eq!(is_prime(1), false);
	assert_eq!(is_prime(2), true);
	assert_eq!(is_prime(4), false);
	assert_eq!(is_prime(5), true);
	assert_eq!(is_prime(13), true);
	assert_eq!(is_prime(1992), false);
}