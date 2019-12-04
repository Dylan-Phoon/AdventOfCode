use std::fs;

fn main() {
	let input = String::from("100000000-1000000000");//fs::read_to_string("input.txt").expect("could not read input.txt");
	let starting_nums: Vec<&str> = input.split("-").collect();

	//let part_one_solution = part_one(starting_nums[0], starting_nums[1]);
	//println!("Part one solution: {:?}", part_one_solution);

	let part_two_solution = part_two(starting_nums[0], starting_nums[1]);
	println!("Part two solution {:?}", part_two_solution);
}

fn part_one(min: &str, max: &str) -> u64 {
	let mut total_solutions: u64 = 0;
	let arr_num: Vec<u64> = min.chars().map(|char| char.to_digit(10).unwrap() as u64).collect();
	
	let (mut curr, mut max) = (min.parse::<u64>().unwrap(), max.parse::<u64>().unwrap());
	while curr < max {
		curr += 1;
		let arr_char_curr: Vec<char> = curr.to_string().chars().collect();
		let (mut is_solution, mut min_tmp) = (false, 0);
		for i in 0..arr_char_curr.len() {
			if arr_char_curr[i] == *arr_char_curr.get(i+1).unwrap_or(&'a') {
				is_solution = true;
			}

			let curr_num_tmp = arr_char_curr[i].to_digit(10).unwrap();
			if curr_num_tmp < min_tmp {
				is_solution = false;
				break;
			}
			min_tmp = curr_num_tmp;
		}

		if is_solution { total_solutions += 1 }
	}

	return total_solutions;
}

fn part_two(min: &str, max: &str) -> u64 {
	let mut total_solutions: u64 = 0;
	let arr_num: Vec<u64> = min.chars().map(|char| char.to_digit(10).unwrap() as u64).collect();
	
	let (mut curr, mut max) = (min.parse::<u64>().unwrap(), max.parse::<u64>().unwrap());
	while curr < max {
		curr += 1;
		if curr % 1000000 == 0 {
			print!("{}[2J", 27 as char);
			println!("{:?}% done...", (curr as f64 / (max - min.parse::<u64>().unwrap()) as f64) as f64);
		}
		let arr_char_curr: Vec<char> = curr.to_string().chars().collect();
		let (mut is_solution, mut min_tmp) = (false, 0);
		for i in 0..arr_char_curr.len() {
			if arr_char_curr[i] != *arr_char_curr.get(i+2).unwrap_or(&'a') &&
				arr_char_curr[i] != *arr_char_curr.get(i.checked_sub(1).unwrap_or(arr_char_curr.len())).unwrap_or(&'a') &&
				  arr_char_curr[i] == *arr_char_curr.get(i+1).unwrap_or(&'a') {
				  	is_solution = true;
			}

			let curr_num_tmp = arr_char_curr[i].to_digit(10).unwrap();
			if curr_num_tmp < min_tmp {
				is_solution = false;
				break;
			}
			min_tmp = curr_num_tmp;
		}

		if is_solution { total_solutions += 1 }
	}

	return total_solutions;
}