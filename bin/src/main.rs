use std::iter::successors;
use std::fs;

fn main() {
	let input = fs::read_to_string("input/input.txt")
		.expect("Something went wrong reading the file");

	let result = calculate_fuel(&input);
	println!("Total fuel: {:?}", result);
}


fn calculate_fuel(input: &String) -> u64 {
	//Total fuel
	input.lines().map(|module| module.parse::<u64>().unwrap() / 3 - 2)
		.map(|initial_fuel: u64| -> u64 {
			//Total module fuel
			successors(Some(initial_fuel), |i| (i / 3).checked_sub(2)).sum();
		}).sum()
}
