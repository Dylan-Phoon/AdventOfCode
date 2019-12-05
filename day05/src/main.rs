use std::{fs, io};

const ADD: i32 = 1;
const MUL: i32 = 2; //multiplies 
const INPUT: i32 = 3; //takes in some input and stores it at the parameter address
const OUTPUT: i32 = 4; //takes in one parameter and prints the value at that address
const JUMP_TRUE: i32 = 5; //if bytes[ip+1] == 1 jump to bytes[ip+2] (do param type check for jump dest)
const JUMP_FALSE: i32 = 6; //if bytes[ip+1] != 1 jump to bytes[ip+2] (do param type check for jump dest)
const LT: i32 = 7; //If bytes[ip+1] < bytes[ip+2] then bytes[bytes[ip+3]] = 1 else 0 less than
const EQ: i32 = 8; //If bytes[ip+1] == bytes[ip+2] then bytes[bytes[ip+3]] = 1 else 0 less than

const HALT: i32 = 99;

//op codes are stored like this;
//1002 which is read left to right [1]-> 2nd param mode [0]-> 1st param mode [02] -> op code
const POSITION: usize = 0; //Go to position to get value 44 = go to address 44
const IMMEDIATE: usize = 1; //Read value literally 44 = 44

fn main() {
	let input = fs::read_to_string("input.txt")
						.expect("could not open input file");
	let bytes: Vec<i32> = input.split(",").map(|x| x.parse::<i32>().unwrap()).collect();
	
	println!("Running Part One: ");
	part_one(bytes.clone(), 1);

	println!("\nRunning Part Two: ");
	part_two(bytes.clone(), 5);
}

fn get_value (param: i32, param_mode: Option<&usize>, bytes: &Vec<i32>) -> i32 {
	let usize_param_mode = param_mode.unwrap_or(&0);
	if *usize_param_mode == POSITION { return bytes[param as usize]; }
	else if *usize_param_mode == IMMEDIATE { return param }
	else { panic!("Invalid Parameter Mode: {:?}", *usize_param_mode); }
}

struct int_machine {
	ip: usize,
	op_code: usize,
	full_op_arr: Vec<usize>,
	bytes: Vec<i32>,
}

impl int_machine {
	pub fn new(bytes: Vec<i32>) -> Self {
		Self {
			ip: 0,
			op_code: 0,
			full_op_arr: Vec::new(),
			bytes
		}
	}

	// Offset is the parameter location distance from the current ip
	fn get_value(&self, offset: usize) -> i32 {
		let param_mode = self.full_op_arr.get(offset + 1).unwrap_or(&0); //+1 because the first two items are the op code itself (see below)
		let param = self.bytes[self.ip + offset];
		if *param_mode == POSITION { return self.bytes[param as usize]; }
		else if *param_mode == IMMEDIATE { return param }
		else { panic!("Invalid Parameter Mode: {:?}", *param_mode); }
	}
}

fn part_two(mut bytes: Vec<i32>, input: i32) -> i32 {
	let mut im = int_machine::new(bytes);
	while im.ip < im.bytes.len() {
		let op_code = im.bytes[im.ip] % 100; //1002 % 100 == 2 and 2 % 100 == 2
		//Get the integer (1002), convert to string ("1002"), split it (['1','0','0','2']),
        //then reverse it (['2','0','0','1'])
		im.full_op_arr = im.bytes[im.ip].to_string().chars().rev().map(|chr| chr.to_digit(10).unwrap() as usize).collect();
		match op_code {
			INPUT => {
				let mstore_addr = im.bytes[im.ip+1] as usize;
				im.bytes[mstore_addr] = input;
				im.ip += 2;
			}
			ADD => {
				let (num_1, num_2) = (im.get_value(1), im.get_value(2)); 
				let mstore_addr = im.bytes[im.ip + 3] as usize;
				im.bytes[mstore_addr] = num_1 + num_2;
				im.ip += 4;
			},
			MUL => {
				let (num_1, num_2) = (im.get_value(1), im.get_value(2)); 
				let mstore_addr = im.bytes[im.ip + 3] as usize;
				im.bytes[mstore_addr] = num_1 * num_2;
				im.ip += 4;	
			},
			OUTPUT => {
				let mstore_addr = im.bytes[im.ip + 1] as usize;
				println!("Output: {:?}", im.bytes[mstore_addr]);
				im.ip += 2;
			},
			JUMP_TRUE => {
				let jump_check = im.get_value(1);
				if jump_check > 0 {
					im.ip = im.get_value(2) as usize;
				} else { im.ip += 3; }
			},
			JUMP_FALSE => {
				let jump_check = im.get_value(1);
				if jump_check == 0 {
					im.ip = im.get_value(2) as usize;
				} else { im.ip += 3; }
			}, 
			LT => {
				let (num_1, num_2) = (im.get_value(1), im.get_value(2));
				let mstore_addr = im.bytes[im.ip+3] as usize;
				im.bytes[mstore_addr] = {if num_1 < num_2 { 1 } else { 0 }};
				im.ip += 4;
			},
			EQ => {
				let (num_1, num_2) = (im.get_value(1), im.get_value(2));
				let mstore_addr = im.bytes[im.ip+3] as usize;
				im.bytes[mstore_addr] = {if num_1 == num_2 { 1 } else { 0 }};
				im.ip += 4;
			},
			HALT => return im.bytes[0],
			invalid => panic!("Error parsing bytes. OP: {:?}", invalid)
		}
	}
	return 0;
}

fn part_one(mut bytes: Vec<i32>, input: i32) -> i32 {
	let mut ip: usize = 0;
	while ip < bytes.len() {
		let op_code = bytes[ip] % 100;
		let full_op_arr: Vec<usize> = bytes[ip].to_string().chars().rev().map(|chr| chr.to_digit(10).unwrap() as usize).collect();
		match op_code {
			INPUT => {
				let mstore_addr = bytes[ip+1] as usize;
				bytes[mstore_addr] = input;
				ip += 2;
			}
			ADD => {
				let (num_1, num_2) = (get_value(bytes[ip+1], full_op_arr.get(2), &bytes), get_value(bytes[ip+2], full_op_arr.get(3), &bytes)); 
				let mstore_addr = bytes[ip + 3] as usize;
				bytes[mstore_addr] = num_1 + num_2;
				ip += 4;
			},
			MUL => {
				let (num_1, num_2) = (get_value(bytes[ip+1], full_op_arr.get(2), &bytes), get_value(bytes[ip+2], full_op_arr.get(3), &bytes)); 
				let mstore_addr = bytes[ip + 3] as usize;
				bytes[mstore_addr] = num_1 * num_2;
				ip += 4;	
			},
			OUTPUT => {
				let mstore_addr = bytes[ip + 1] as usize;
				println!("Output: {:?}", bytes[mstore_addr]);
				ip += 2;
			},
			HALT => return bytes[0],
			invalid => panic!("Error parsing bytes. OP: {:?}", invalid)
		}
	}
	return 0;
}

