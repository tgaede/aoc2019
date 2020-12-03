// use crate::common::Solution;
// use crate::intcode::{parse_program, run_computer, IntcodeComputer, Word};
//
// use std::collections::VecDeque;
// use std::convert::TryFrom;
// use std::iter::FromIterator;

// const LEFT: char = '<';
// const RIGHT: char = '>';
// const UP: char = '^';
// const DOWN: char = 'v';
//
// #[derive(Clone, Debug)]
// struct Move {
//     direction: char,
//     distance: u8,
// }
//
// impl Move {
//     fn new(c: char, d: u8) -> Move {
//         Move {
//             direction: c,
//             distance: d,
//         }
//     }
// }
//
// pub fn solve(lines: &[String]) -> Solution {
//     let mut computer = IntcodeComputer::new(parse_program(lines));
//
//     run_computer(&mut computer);
//     let picture: Vec<Vec<char>> = parse_picture(&computer.output);
//     print_ascii(&picture);
//
//     let part1: String = solve_part1(lines);
//     let part2: String = solve_part2(lines);
//
//     (part1, part2)
// }
//
// fn print_ascii(data: &Vec<Vec<char>>) {
//     data.iter()
//         .for_each(|x| println!("{}", x.into_iter().collect::<String>()));
// }
//
// fn parse_picture(data: &VecDeque<Word>) -> Vec<Vec<char>> {
//     let mut picture: Vec<Vec<char>> = Vec::new();
//
//     Vec::from_iter(
//         data.into_iter()
//             .map(|x| std::char::from_u32(u32::try_from(*x).unwrap()).unwrap()),
//     )
//     .split(|x| *x == '\n')
//     .for_each(|x| picture.push(Vec::from(x)));
//
//     picture
// }
//
// fn solve_part1(lines: &[String]) -> String {
//     let mut computer = IntcodeComputer::new(parse_program(lines));
//
//     run_computer(&mut computer);
//     sum_alignment_parameters(&parse_picture(&computer.output)).to_string()
// }
//
// fn sum_alignment_parameters(data: &Vec<Vec<char>>) -> usize {
//     let mut row: usize;
//     let mut col: usize;
//     let mut total: usize = 0;
//
//     row = 1;
//     while data.len() != 0 && row < (data.len() - 1) {
//         col = 1;
//         while data[row].len() != 0 && col < (data[row].len() - 1) {
//             if data[row][col] == '#'
//                 && data[row - 1][col] == '#'
//                 && col < data[row + 1].len()
//                 && data[row + 1][col] == '#'
//                 && data[row][col - 1] == '#'
//                 && (col + 1) < data[row].len()
//                 && data[row][col + 1] == '#'
//             {
//                 total += row * col;
//             }
//
//             col += 1;
//         }
//
//         row += 1;
//     }
//
//     total
// }
//
// fn solve_part2(_lines: &[String]) -> String {
//     // let mut computer = IntcodeComputer::new(parse_program(lines));
//     let result: String = String::new();
//
//     // get path
//     // let mut path: String;
//
//     // find largest substrings
//
//     // set input to 2
//     // run_computer(&mut computer);
//
//     // input main movement routine
//
//     // input each movement A, B, C
//
//     // continuous video feed?
//
//     result
// }
