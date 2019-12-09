use std::fs;
use std::thread;
use std::sync::mpsc;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};



const ADD: i64 = 1;
const MUL: i64 = 2; //multiplies 
const INPUT: i64 = 3; //takes in some input and stores it at the parameter address
const OUTPUT: i64 = 4; //takes in one parameter and prints the value at that address
const JUMP_TRUE: i64 = 5; //if bytes[ip+1] == 1 jump to bytes[ip+2] (do param type check for jump dest)
const JUMP_FALSE: i64 = 6; //if bytes[ip+1] != 1 jump to bytes[ip+2] (do param type check for jump dest)
const LT: i64 = 7; //If bytes[ip+1] < bytes[ip+2] then bytes[bytes[ip+3]] = 1 else 0 less than
const EQ: i64 = 8; //If bytes[ip+1] == bytes[ip+2] then bytes[bytes[ip+3]] = 1 else 0 less than
const REL_ADJUST: i64 = 9;

const HALT: i64 = 99;

//op codes are stored like this;
//1002 which is read left to right [1]-> 2nd param mode [0]-> 1st param mode [02] -> op code
const POSITION: usize = 0; //Go to position to get value 44 = go to address 44
const IMMEDIATE: usize = 1; //Read value literally 44 = 44
const RELATIVE: usize = 2;

fn main() {
	let input = fs::read_to_string("input.txt")
						.expect("could not open input file");
	let bytes: Vec<i64> = input.split(",").map(|x| x.parse::<i64>().unwrap()).collect();
	println!("\nRunning Part One: ");
	let part_one_solution = part_one(bytes.clone());
	println!("Part one solution: {:?}", part_one_solution);

	println!("\nRunning Part Two: ");
	let part_two_solution = part_two(bytes.clone());
	println!("Part two solution: {:?}", part_two_solution);
}

struct IntMachine {
	ip: usize,
	op_code: i64,
	full_op_arr: Vec<usize>,
	bytes: Vec<i64>,
	tx: mpsc::Sender<i64>, //Thread to send all output
	rx: mpsc::Receiver<i64>, //Thread to receive all input
	last_output: i64,
	relative_base: i64,
	heap: HashMap<i64, i64>,
	name: u8
}

impl IntMachine {
	pub fn new(bytes: Vec<i64>, rx: mpsc::Receiver<i64>, tx: mpsc::Sender<i64>, name: u8) -> Self { 
		Self {
			ip: 0,
			op_code: 0,
			full_op_arr: Vec::new(),
			bytes,
			name,
			tx,
			rx,
			heap: HashMap::new(), //<K: Address, V: Value>
			relative_base: 0,
			last_output: 0
		}
	}

	fn heap_or_stack_access<'a>(&'a mut self, address: i64) -> Option<&'a mut i64> {
		if address < 0 {
			return None;
		}

		if address >= self.bytes.len() as i64 {
			if !self.heap.contains_key(&address) {
				self.heap.insert(address, 0);
			}
			return Some(self.heap.get_mut(&address).unwrap());
		} else {
			return Some(self.bytes.get_mut(address as usize).unwrap());
		}
	}

	// Offset is the parameter location distance from the current ip
	fn get_value<'a> (&'a mut self, offset: usize) -> &'a mut i64 {
		//offset+1 because the first two items are the op code itself 
		//(full_op_arr is op code split into it's individual digits in reverse stored as a vector)
		//If there is no digit there, then we can assume the default param mdoe is POSITION
		let param_mode = self.full_op_arr.get(offset + 1).unwrap_or(&POSITION);
		let param = *self.bytes.get(self.ip + offset).unwrap();
		match *param_mode {
			POSITION => return self.heap_or_stack_access(param.clone()).unwrap(),
			IMMEDIATE => return self.bytes.get_mut(self.ip + offset).unwrap(),
			RELATIVE => return self.heap_or_stack_access(param.clone() + self.relative_base).unwrap(),
			invalid => panic!("Invalid parameter mode: {:?}", invalid)
		}
	}

	fn input(&mut self) {
		let input = self.rx.recv().unwrap();
		println!("Asking input for:  {:?}  at:  {:?}  and got:  {:?}", self.name as char,  self.ip, input);
		
		let mstore_location = self.get_value(1);
		*mstore_location = input;
		self.ip += 2;		
	}

	fn output(&mut self) {
		//Send the output to the machine target thread
		let output = *self.get_value(1);
		self.last_output = output;
		println!("Sending output from:  {:?}  at:  {:?}  and sent:  {:?}", self.name as char, self.ip, output);
		self.tx.send(output.clone()).unwrap_or(());
		self.ip += 2;
	}

	fn math(&mut self, op_code: i64) {
		let (num_1, num_2) = (*self.get_value(1), *self.get_value(2)); 
		let mstore_addr = self.get_value(3);
		*mstore_addr = { 
				match op_code {
					ADD => num_1.checked_add(num_2).unwrap(),
					MUL => num_1.checked_mul(num_2).unwrap(),
					invalid => panic!("Invalid op code in fn IntMachine::math: {:?}", invalid)
				}
			};
		self.ip += 4;
	}


	fn jump(&mut self, op_code: i64) {
		let jump_check = *self.get_value(1);
		match op_code {
			JUMP_TRUE => {
				if jump_check > 0 {
					self.ip = *self.get_value(2) as usize;
				} else { self.ip += 3; }
			},
			JUMP_FALSE => {
				if jump_check == 0 {
					self.ip = *self.get_value(2) as usize;
				} else { self.ip += 3; }
			},
			invalid => panic!("Invalid op code in fn IntMachine::jump: {:?}", invalid)
		}
	}

	fn equality_check(&mut self, op_code: i64) {
		let (num_1, num_2) = (*self.get_value(1), *self.get_value(2));
		let mstore_addr = self.get_value(3);
		*mstore_addr = {
			match op_code {
				LT => if num_1 < num_2 { 1 } else { 0 },
				EQ =>  if num_1 == num_2 { 1 } else { 0 },
				invalid => panic!("Invalid op code in fn IntMachine::jump: {:?}", invalid),		
			}
		};
		self.ip += 4;
	}

	fn relative_base_adjust(&mut self) {
		let offset = *self.get_value(1);
		self.relative_base += offset;
		self.ip += 2;
	}

	fn execute(&mut self) -> i64 {
		while self.ip < self.bytes.len() {
			self.op_code = self.bytes[self.ip] % 100; //1002 % 100 == 2 and 2 % 100 == 2
			//Get the integer (1002), convert to string ("1002"), split it (['1','0','0','2']),
	        //then reverse it (['2','0','0','1'])
			self.full_op_arr = self.bytes[self.ip].to_string().chars().rev().map(|chr| chr.to_digit(10).unwrap() as usize).collect();
			match self.op_code {
				INPUT => self.input(),
				OUTPUT => self.output(),
				ADD => self.math(ADD),
				MUL => self.math(MUL),
				JUMP_TRUE => self.jump(JUMP_TRUE),
				JUMP_FALSE => self.jump(JUMP_FALSE), 
				LT => self.equality_check(LT),
				EQ => self.equality_check(EQ),
				REL_ADJUST => self.relative_base_adjust(),
				HALT => {println!("halting {:?}", self.name as char); return self.last_output},
				invalid => panic!("Error parsing bytes. OP: {:?}", invalid)
			}
		}
		return 0;
	}
}

fn part_one(bytes: Vec<i64>) -> i64 {
	let (input_tx, input_rx) = mpsc::channel(); //Thread for giving input to machine
	let (output_tx, output_rx) = mpsc::channel(); //Thread for recieving output from machine
	let name = b'A';
	let final_output = Arc::new(Mutex::new(0 as i64)); 
	let mut machine = IntMachine::new(bytes.clone(), input_rx, output_tx, name);
	let final_output_tmp = Arc::clone(&final_output);
	let machine_thread = thread::spawn(move || {
		*final_output_tmp.lock().unwrap() = machine.execute();
	});

	input_tx.send(1).unwrap();
	machine_thread.join().unwrap();
	return *final_output.lock().unwrap();
}

fn part_two(bytes: Vec<i64>) -> i64 {
	let (input_tx, input_rx) = mpsc::channel(); //Thread for giving input to machine
	let (output_tx, output_rx) = mpsc::channel(); //Thread for recieving output from machine
	let name = b'A';
	let final_output = Arc::new(Mutex::new(0 as i64)); 
	let mut machine = IntMachine::new(bytes.clone(), input_rx, output_tx, name);
	let final_output_tmp = Arc::clone(&final_output);
	let machine_thread = thread::spawn(move || {
		*final_output_tmp.lock().unwrap() = machine.execute();
	});

	input_tx.send(2).unwrap();
	machine_thread.join().unwrap();
	return *final_output.lock().unwrap();
}