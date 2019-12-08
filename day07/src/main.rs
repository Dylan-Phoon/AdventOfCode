use std::{fs, io};
use std::thread;
use std::sync::mpsc;
use std::collections::HashMap;
use std::sync::{Mutex, Arc};



const ADD: i64 = 1;
const MUL: i64 = 2; //multiplies 
const INPUT: i64 = 3; //takes in some input and stores it at the parameter address
const OUTPUT: i64 = 4; //takes in one parameter and prints the value at that address
const JUMP_TRUE: i64 = 5; //if bytes[ip+1] == 1 jump to bytes[ip+2] (do param type check for jump dest)
const JUMP_FALSE: i64 = 6; //if bytes[ip+1] != 1 jump to bytes[ip+2] (do param type check for jump dest)
const LT: i64 = 7; //If bytes[ip+1] < bytes[ip+2] then bytes[bytes[ip+3]] = 1 else 0 less than
const EQ: i64 = 8; //If bytes[ip+1] == bytes[ip+2] then bytes[bytes[ip+3]] = 1 else 0 less than

const HALT: i64 = 99;

//op codes are stored like this;
//1002 which is read left to right [1]-> 2nd param mode [0]-> 1st param mode [02] -> op code
const POSITION: usize = 0; //Go to position to get value 44 = go to address 44
const IMMEDIATE: usize = 1; //Read value literally 44 = 44

fn main() {
	let input = fs::read_to_string("input.txt")
						.expect("could not open input file");
	let bytes: Vec<i64> = input.split(",").map(|x| x.parse::<i64>().unwrap()).collect();
	//println!("\nRunning Part One: ");
	//let part_one_solution = part_one(bytes.clone());
	//println!("{:?}", part_one_solution);

	println!("\nRunning Part Two: ");
	let part_two_solution = part_two(bytes.clone());
	println!("{:?}", part_two_solution);
}

fn get_value (param: i64, param_mode: Option<&usize>, bytes: &Vec<i64>) -> i64 {
	let usize_param_mode = param_mode.unwrap_or(&0);
	if *usize_param_mode == POSITION { return bytes[param as usize]; }
	else if *usize_param_mode == IMMEDIATE { return param }
	else { panic!("Invalid Parameter Mode: {:?}", *usize_param_mode); }
}

struct IntMachine {
	ip: usize,
	op_code: usize,
	full_op_arr: Vec<usize>,
	bytes: Vec<i64>,
	tx: mpsc::Sender<i64>,
	rx: mpsc::Receiver<i64>,
	last_output: i64,
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
			last_output: 0
		}
	}

	// Offset is the parameter location distance from the current ip
	fn get_value(&self, offset: usize) -> i64 {
		let param_mode = self.full_op_arr.get(offset + 1).unwrap_or(&0); //+1 because the first two items are the op code itself (see below)
		let param = self.bytes[self.ip + offset];
		if *param_mode == POSITION { return self.bytes[param as usize]; }
		else if *param_mode == IMMEDIATE { return param }
		else { panic!("Invalid Parameter Mode: {:?}", *param_mode); }
	}

	fn input(&mut self) {
		let mstore_addr = self.bytes[self.ip+1] as usize;

		//Listen for the input on the thread
		let input = self.rx.recv().unwrap();
		println!("Asking input for:  {:?}  at:  {:?}  and got:  {:?}", self.name as char,  self.ip, input);
		self.bytes[mstore_addr] = input;
		self.ip += 2;		
	}

	fn output(&mut self) {
		let mstore_addr = self.bytes[self.ip + 1] as usize;
		println!("Sending output for:  {:?}  at:  {:?}  and sent:  {:?}\n", self.name as char, self.ip, self.bytes[mstore_addr]);
		
		//Send the output to the machine target thread
		let output = self.bytes[mstore_addr];
		self.last_output = output;
		self.tx.send(output.clone()).unwrap_or(());
		self.ip += 2;
	}

	fn math(&mut self, op_code: i64) {
		let (num_1, num_2) = (self.get_value(1), self.get_value(2)); 
		let mstore_addr = self.bytes[self.ip + 3] as usize;
		self.bytes[mstore_addr] = { 
									match op_code {
										ADD => num_1 + num_2,
										MUL => num_1 * num_2,
										invalid => panic!("Invalid op code in fn IntMachine::math: {:?}", invalid)
									}
								};
		self.ip += 4;
	}


	fn jump(&mut self, op_code: i64) {
		let jump_check = self.get_value(1);
		match op_code {
			JUMP_TRUE => {
				if jump_check > 0 {
					self.ip = self.get_value(2) as usize;
				} else { self.ip += 3; }
			},
			JUMP_FALSE => {
				if jump_check == 0 {
					self.ip = self.get_value(2) as usize;
				} else { self.ip += 3; }
			},
			invalid => panic!("Invalid op code in fn IntMachine::jump: {:?}", invalid)
		}
	}

	fn equality_check(&mut self, op_code: i64) {
		let (num_1, num_2) = (self.get_value(1), self.get_value(2));
		let mstore_addr = self.bytes[self.ip+3] as usize;
		self.bytes[mstore_addr] = {
			match op_code {
				LT => if num_1 < num_2 { 1 } else { 0 },
				EQ =>  if num_1 == num_2 { 1 } else { 0 },
				invalid => panic!("Invalid op code in fn IntMachine::jump: {:?}", invalid),		
			}
		};
		self.ip += 4;
	}

	fn execute(&mut self) -> i64 {
		while self.ip < self.bytes.len() {
			let op_code = self.bytes[self.ip] % 100; //1002 % 100 == 2 and 2 % 100 == 2
			//Get the integer (1002), convert to string ("1002"), split it (['1','0','0','2']),
	        //then reverse it (['2','0','0','1'])
			self.full_op_arr = self.bytes[self.ip].to_string().chars().rev().map(|chr| chr.to_digit(10).unwrap() as usize).collect();
			match op_code {
				INPUT => self.input(),
				OUTPUT => self.output(),
				ADD => self.math(ADD),
				MUL => self.math(MUL),
				JUMP_TRUE => self.jump(JUMP_TRUE),
				JUMP_FALSE => self.jump(JUMP_FALSE), 
				LT => self.equality_check(LT),
				EQ => self.equality_check(EQ),
				HALT => {println!("halting {:?}", self.name as char); return self.last_output},
				invalid => panic!("Error parsing bytes. OP: {:?}", invalid)
			}
		}
		return 0;
	}
}

fn part_two(mut bytes: Vec<i64>) -> i64 {
	let curr_output = Arc::new(Mutex::new(0));
	let mut max_output = Arc::new(Mutex::new(0));
	let mut phase_settings: Vec<i64> = vec![5,6,7,8,9];

	let orig_arr = phase_settings.clone();
	let mut initialize = true;

	while phase_settings != orig_arr || initialize {
		initialize = false;
		for i in 0..phase_settings.len() - 1 {
			phase_settings.swap(i, i+1);
			println!("Running machines with phase settings: {:?}", phase_settings);

			//Create a hashmap of channels so that each machine has a recv and send
			let mut machine_recs: HashMap<u8, mpsc::Receiver<i64>> = HashMap::new();
			let mut machine_txs: HashMap<u8, mpsc::Sender<i64>> = HashMap::new();
			for id in b'A'..b'F' {
				let (tx, rx) = mpsc::channel();
				machine_recs.insert(id, rx);
				machine_txs.insert(id, tx);
			}

			let mut handles = Vec::new();
			for id in b'A'..b'F' {
				let target = {if id == b'E' { b'A' } else { id + 1 }};
				let rx = machine_recs.remove(&id).unwrap();
				let tx = machine_txs.get(&target).unwrap();
				let mut amplifier = IntMachine::new(bytes.clone(), rx, mpsc::Sender::clone(&tx), id);
				let mut tmp_curr_output = Arc::clone(&curr_output);
				let mut tmp_max_output = Arc::clone(&max_output);
				let handle = thread::spawn(move || {
					*tmp_curr_output.lock().unwrap() = amplifier.execute();
					if *tmp_curr_output.lock().unwrap() > *tmp_max_output.lock().unwrap() {
						*tmp_max_output.lock().unwrap() = *tmp_curr_output.lock().unwrap();
					}
				});

				let curr_tx = machine_txs.get(&id).unwrap();
				curr_tx.send(phase_settings[(b'E' - id) as usize]).unwrap();
				println!("{:?}", b'E' - id);

				if id == b'E' {
					tx.send(0);
				}

				handles.push(handle);
			}

			//Join all of the threads
			for handle in handles {
				handle.join().unwrap();
			}

			let mut tmp_curr_output = *curr_output.lock().unwrap();
			println!("tmp: {:?}", tmp_curr_output);
			tmp_curr_output = 0;
		}
	}

	return *max_output.lock().unwrap();
}

/*
fn part_one(mut bytes: Vec<i64>) -> i64 {
	let mut curr_output = 0;
	let mut phase_settings: Vec<i64> = vec![0,1,2,3,4];
	let mut max_output = 0;

	let orig_arr = phase_settings.clone();
	let mut initialize = true;

	while phase_settings != orig_arr || initialize {
		initialize = false;
		for i in 0..phase_settings.len() - 1 {
			phase_settings.swap(i, i+1);
			println!("Running machines {:?}", phase_settings);
			let (tx, rx) = mpsc::channel();
			let machine_channel = MachineChannel {tx, rx};
			for id in 0..=(b'E'-b'A') {
 				let mut amplifier = IntMachine::new(bytes.clone(), machine_channel, id as u32);
				curr_output = amplifier.execute();
			}
			if curr_output > max_output {
				max_output = curr_output;
			}
			curr_output = 0;
		}
	}

	return max_output;
}
*/