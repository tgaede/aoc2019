// use std::collections::VecDeque;
//
// pub type Word = i64;
// type Memory = Vec<Word>;
//
// #[derive(Clone, Debug)]
// pub struct IntcodeComputer {
//     pub eip: usize,
//     pub prog: Memory,
//     relbase: Word,
//     pub input: VecDeque<Word>,
//     pub output: VecDeque<Word>,
// }
//
// const OP_ADD: Word = 1;
// const OP_MULTIPLY: Word = 2;
// const OP_INPUT: Word = 3;
// const OP_OUTPUT: Word = 4;
// const OP_JUMP_NONZERO: Word = 5;
// const OP_JUMP_ZERO: Word = 6;
// const OP_LESS: Word = 7;
// const OP_EQ: Word = 8;
// const OP_RELBASE: Word = 9;
// const OP_HALT: Word = 99;
//
// impl IntcodeComputer {
//     pub fn new(program: Vec<Word>) -> IntcodeComputer {
//         IntcodeComputer {
//             eip: 0,
//             prog: program,
//             relbase: 0,
//             input: VecDeque::new(),
//             output: VecDeque::new(),
//         }
//     }
// }
//
// pub fn step_computer(computer: &mut IntcodeComputer) {
//     let instruction = computer.prog[computer.eip];
//     let opcode = instruction % 100;
//     let eip = computer.eip;
//     let relbase = computer.relbase;
//
//     let get_addr = |prog: &mut Memory, offset: usize| -> usize {
//         let parmode_pow = match offset {
//             1 => 100,
//             2 => 1000,
//             3 => 10000,
//             _ => unreachable!(),
//         };
//         let out_addr = match (instruction / parmode_pow) % 10 {
//             0 => (prog[eip + offset]) as usize,
//             1 => eip + offset,
//             2 => (relbase + prog[eip + offset]) as usize,
//             _ => unreachable!(),
//         };
//         if out_addr >= prog.len() {
//             prog.resize(out_addr + 1, 0);
//         }
//         out_addr
//     };
//
//     let get_arg = |prog: &mut Memory, arg_num: usize| -> Word {
//         let addr = get_addr(prog, arg_num);
//         prog[addr]
//     };
//
//     computer.eip = match opcode {
//         OP_ADD => {
//             let io = get_addr(&mut computer.prog, 3);
//             computer.prog[io] = get_arg(&mut computer.prog, 1) + get_arg(&mut computer.prog, 2);
//             computer.eip + 4
//         }
//
//         OP_MULTIPLY => {
//             let io = get_addr(&mut computer.prog, 3);
//             computer.prog[io] = get_arg(&mut computer.prog, 1) * get_arg(&mut computer.prog, 2);
//             computer.eip + 4
//         }
//
//         OP_INPUT => {
//             let io = get_addr(&mut computer.prog, 1);
//             if let Some(i) = computer.input.pop_front() {
//                 computer.prog[io] = i;
//                 computer.eip + 2
//             } else {
//                 computer.eip
//             }
//         }
//
//         OP_OUTPUT => {
//             computer.output.push_back(get_arg(&mut computer.prog, 1));
//             computer.eip + 2
//         }
//
//         OP_JUMP_NONZERO => {
//             if get_arg(&mut computer.prog, 1) != 0 {
//                 get_arg(&mut computer.prog, 2) as usize
//             } else {
//                 computer.eip + 3
//             }
//         }
//
//         OP_JUMP_ZERO => {
//             if get_arg(&mut computer.prog, 1) == 0 {
//                 get_arg(&mut computer.prog, 2) as usize
//             } else {
//                 computer.eip + 3
//             }
//         }
//
//         OP_LESS => {
//             let io = get_addr(&mut computer.prog, 3);
//             computer.prog[io] = if get_arg(&mut computer.prog, 1) < get_arg(&mut computer.prog, 2) {
//                 1
//             } else {
//                 0
//             };
//             computer.eip + 4
//         }
//
//         OP_EQ => {
//             let io = get_addr(&mut computer.prog, 3);
//             computer.prog[io] = if get_arg(&mut computer.prog, 1) == get_arg(&mut computer.prog, 2)
//             {
//                 1
//             } else {
//                 0
//             };
//             computer.eip + 4
//         }
//
//         OP_RELBASE => {
//             computer.relbase += get_arg(&mut computer.prog, 1);
//             computer.eip + 2
//         }
//
//         OP_HALT => computer.eip,
//         _ => unreachable!(),
//     };
// }
//
// pub fn run_computer(computer: &mut IntcodeComputer) {
//     while is_computer_running(computer)
//         && !(computer.input.is_empty() && computer_expects_input(computer))
//     {
//         step_computer(computer);
//     }
// }
//
// pub fn is_computer_running(computer: &IntcodeComputer) -> bool {
//     computer.prog[computer.eip] != OP_HALT
// }
//
// pub fn computer_expects_input(computer: &IntcodeComputer) -> bool {
//     computer.prog[computer.eip] % 100 == OP_INPUT
// }
//
// pub fn parse_program(lines: &[String]) -> Vec<Word> {
//     lines[0].split(',').map(|s| s.parse().unwrap()).collect()
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_quine() {
//         let quine: Vec<Word> = vec![
//             109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
//         ];
//         let mut computer = IntcodeComputer::new(quine.clone());
// 		run_computer(&mut computer);
// 		println!("input: {:#?}", computer.input);
// 		println!("output: {:#?}", computer.output);
// 		assert!(!computer.output.is_empty());
//         assert_eq!(computer.output, quine);
//     }
//
// 	#[test]
// 	fn test_16_digits() {
// 		let mut computer = IntcodeComputer::new(vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0]);
// 		run_computer(&mut computer);
// 		println!("input: {:#?}", computer.input);
// 		println!("output: {:#?}", computer.output);
// 		assert!(!computer.output.is_empty());
// 		assert_eq!(computer.output[0].to_string().len(), 16);
// 	}
//
// 	#[test]
// 	fn test_big_number() {
// 		let program = vec![104, 1125899906842624, 99];
// 		let expect = program[1];
// 		let mut computer = IntcodeComputer::new(program);
// 		run_computer(&mut computer);
// 		println!("input: {:#?}", computer.input);
// 		println!("output: {:#?}", computer.output);
// 		assert!(!computer.output.is_empty());
// 		assert_eq!(computer.output[0], expect);
// 	}
// }
