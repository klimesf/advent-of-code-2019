use std::collections::{HashMap, VecDeque};
use std::fs;
use itertools::Itertools;

pub(crate) fn day21() {
    let input = fs::read_to_string("input/day21/input.txt").unwrap();
    let code: Vec<i64> = input.trim().split(',').map(|c| c.parse().unwrap()).collect();

    part_a(&code);
    part_b(&code);
}

fn part_a(code: &Vec<i64>) {
    // patterns
    // #####.###########
    // #####..#.########
    // #####...#########

    let mut input = VecDeque::new();

    "NOT C J\n".chars().for_each(|c| input.push_back(c as i64)); // If there is a gap on the 3rd
    "AND D J\n".chars().for_each(|c| input.push_back(c as i64)); // but hull on 4th place => we can jump
    "NOT A T\n".chars().for_each(|c| input.push_back(c as i64)); // Or if there is a gap immediately in front of us
    "OR T J\n".chars().for_each(|c| input.push_back(c as i64));  // => we can jump

    "WALK\n".chars().for_each(|c| input.push_back(c as i64));

    let mut intcode = intcode_instance(&code);
    intcode.run(&mut input);

    println!("{}", intcode.output.iter().map(|c| *c as u8 as char).join(""));
    println!("{}", *intcode.output.last().unwrap() as i64);
}

fn part_b(code: &Vec<i64>) {
    // patterns
    // #####.##.#.##.###
    // #####..#.########
    // #####...#########

    let mut input = VecDeque::new();

    "NOT C T\n".chars().for_each(|c| input.push_back(c as i64)); // If there is a gap on 3rd place
    "AND D T\n".chars().for_each(|c| input.push_back(c as i64)); // but a hull on 4th place
    "AND H J\n".chars().for_each(|c| input.push_back(c as i64)); // and on 8th place,
    "OR T J\n".chars().for_each(|c| input.push_back(c as i64));  // we can jump

    "NOT B T\n".chars().for_each(|c| input.push_back(c as i64)); // If there is a gap on 2nd place
    "AND D T\n".chars().for_each(|c| input.push_back(c as i64)); // but a hull on 4th place
    "AND H J\n".chars().for_each(|c| input.push_back(c as i64)); // and on 8th place,
    "OR T J\n".chars().for_each(|c| input.push_back(c as i64));  // we can jump

    "NOT A T\n".chars().for_each(|c| input.push_back(c as i64)); // Or if there is a gap immediately in front of us
    "OR T J\n".chars().for_each(|c| input.push_back(c as i64));  // => we can jump

    "RUN\n".chars().for_each(|c| input.push_back(c as i64));

    let mut intcode = intcode_instance(&code);
    intcode.run(&mut input);

    println!("{}", intcode.output.iter().map(|c| *c as u8 as char).join(""));
    println!("{}", *intcode.output.last().unwrap() as i64);
}

struct IntcodeProcessor {
    instruction_ptr: i64,
    memory: HashMap<i64, i64>,
    relative_base: i64,
    output: Vec<i64>,
}

impl IntcodeProcessor {
    fn run(&mut self, input: &mut VecDeque<i64>) {
        loop {
            let _instr = self.memory[&self.instruction_ptr];
            let opcode = self.memory[&self.instruction_ptr] % 100;
            let a_mode = (self.memory[&self.instruction_ptr] / 100 % 10) as i32;
            let b_mode = (self.memory[&self.instruction_ptr] / 1000 % 10) as i32;
            let c_mode = (self.memory[&self.instruction_ptr] / 10000 % 10) as i32;

            match opcode {
                1 => {
                    let a = self.memory[&(self.instruction_ptr + 1)];
                    let b = self.memory[&(self.instruction_ptr + 2)];
                    let c = self.memory[&(self.instruction_ptr + 3)];

                    let a_val = self.mem_read(a_mode, a);
                    let b_val = self.mem_read(b_mode, b);

                    self.mem_write(c_mode, c, a_val + b_val);
                    self.instruction_ptr += 4;
                }
                2 => {
                    let a = self.memory[&(self.instruction_ptr + 1)];
                    let b = self.memory[&(self.instruction_ptr + 2)];
                    let c = self.memory[&(self.instruction_ptr + 3)];

                    let a_val = self.mem_read(a_mode, a);
                    let b_val = self.mem_read(b_mode, b);

                    self.mem_write(c_mode, c, a_val * b_val);
                    self.instruction_ptr += 4;
                }
                3 => {
                    let a = self.memory[&(self.instruction_ptr + 1)];
                    let inp_val = input.pop_front().unwrap();
                    self.mem_write(a_mode, a, inp_val);
                    self.instruction_ptr += 2;
                }
                4 => {
                    let a = self.memory[&(self.instruction_ptr + 1)];
                    let a_val = self.mem_read(a_mode, a);
                    self.output.push(a_val);
                    self.instruction_ptr += 2;
                }
                5 => {
                    let a = self.memory[&(self.instruction_ptr + 1)];
                    let b = self.memory[&(self.instruction_ptr + 2)];
                    let a_val = self.mem_read(a_mode, a);
                    if a_val != 0 {
                        let b_val = self.mem_read(b_mode, b);
                        self.instruction_ptr = b_val;
                    } else {
                        self.instruction_ptr += 3;
                    }
                }
                6 => {
                    let a = self.memory[&(self.instruction_ptr + 1)];
                    let b = self.memory[&(self.instruction_ptr + 2)];
                    let a_val = self.mem_read(a_mode, a);
                    if a_val == 0 {
                        let b_val = self.mem_read(b_mode, b);
                        self.instruction_ptr = b_val;
                    } else {
                        self.instruction_ptr += 3;
                    }
                }
                7 => {
                    let a = self.memory[&(self.instruction_ptr + 1)];
                    let b = self.memory[&(self.instruction_ptr + 2)];
                    let c = self.memory[&(self.instruction_ptr + 3)];

                    let a_val = self.mem_read(a_mode, a);
                    let b_val = self.mem_read(b_mode, b);

                    self.mem_write(c_mode, c, if a_val < b_val { 1 } else { 0 });
                    self.instruction_ptr += 4;
                }
                8 => {
                    let a = self.memory[&(self.instruction_ptr + 1)];
                    let b = self.memory[&(self.instruction_ptr + 2)];
                    let c = self.memory[&(self.instruction_ptr + 3)];

                    let a_val = self.mem_read(a_mode, a);
                    let b_val = self.mem_read(b_mode, b);

                    self.mem_write(c_mode, c, if a_val == b_val { 1 } else { 0 });
                    self.instruction_ptr += 4;
                }
                9 => {
                    let a = self.memory[&(self.instruction_ptr + 1)];
                    let a_val = self.mem_read(a_mode, a);
                    self.relative_base += a_val;
                    self.instruction_ptr += 2;
                }
                99 => { break; }
                _ => { panic!("Unknown opcode {} at pos {}", opcode, self.instruction_ptr); }
            }
        }
    }

    fn mem_read(&mut self, mode: i32, val: i64) -> i64 {
        match mode {
            0 => {
                if val < 0 { panic!("Invalid memory address: {}", val) }
                *self.memory.entry(val).or_insert(0)
            }
            1 => val,
            2 => {
                let addr = val + self.relative_base;
                if addr < 0 { panic!("Invalid memory address: {}", addr) }
                *self.memory.entry(addr).or_insert(0)
            }
            _ => panic!("Unknown mode: {}", mode),
        }
    }

    fn mem_write(&mut self, mode: i32, addr: i64, val: i64) {
        match mode {
            0 => {
                if addr < 0 { panic!("Invalid memory address: {}", addr); }
                self.memory.insert(addr, val);
            }
            1 => panic!("How to write in mode 1?"),
            2 => {
                let i = addr + self.relative_base;
                if i < 0 { panic!("Invalid memory address: {}", i) }
                self.memory.insert(i, val);
            }
            _ => panic!("Unknown mode: {}", mode),
        };
    }
}

fn intcode_instance(code: &Vec<i64>) -> IntcodeProcessor {
    let mut memory = HashMap::new();
    for i in 0..code.len() {
        memory.insert(i as i64, code[i]);
    }
    IntcodeProcessor {
        instruction_ptr: 0,
        memory,
        relative_base: 0,
        output: vec!(),
    }
}
