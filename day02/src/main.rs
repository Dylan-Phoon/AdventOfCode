use std::fs;

const ADD: usize = 1;
const MUL: usize = 2;
const HALT: usize = 99;

fn main() {
	let input = fs::read_to_string("input.txt")
						.expect("could not open input file");

	let bytes: Vec<usize> = input.split(",").map(|x| x.parse::<usize>().unwrap()).collect();
	let res = part_one(bytes.clone());
	let res2 = part_two(bytes.clone(), 19690720);
	println!("Part one result: {:?}", res);
	println!("Part two result: {:?}", res2);
}

fn part_one(mut bytes: Vec<usize>) -> usize {
	let mut ip: usize = 0;
	bytes[1] = 12;
	bytes[2] = 2;
	while ip < bytes.len() {
		let mstore_addr: usize = {if ip != HALT {bytes[ip+3]} else {0}}; 
		match bytes[ip] {
			ADD => bytes[mstore_addr] = bytes[bytes[ip+1]] + bytes[bytes[ip+2]],
			MUL => bytes[mstore_addr] = bytes[bytes[ip+1]] * bytes[bytes[ip+2]],
			HALT => return bytes[0],
			err => panic!("Error parsing bytes {:?}", err)
		}
		ip += 4
	}

	return 0
}

fn part_two(bytes: Vec<usize>, target_output: usize) -> Result<usize, &'static str> {
	for noun in 0..100 {
		for verb in 0..100 {
			let mut ip: usize = 0;
			let mut b = bytes.clone();
			b[1] = noun;
			b[2] = verb;
			while ip < b.len() {
				let mstore_addr: usize = { if ip+3 < bytes.len() { b[ip+3] } else { 0 }};
				match b[ip] {
					ADD => b[mstore_addr] = b[b[ip+1]] + b[b[ip+2]],
					MUL => b[mstore_addr] = b[b[ip+1]] * b[b[ip+2]],
					HALT => if b[0] == target_output { return Ok(100 * noun + verb) } else { break; },
					_ => break
				}
				ip += 4;
			}
		}
	}
	return Err("Could not find verb or noun to satisfy output");
}