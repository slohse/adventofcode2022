use std::fs::File;
use std::io::prelude::*;
use clap::Parser;
use std::path::PathBuf;
use regex::Regex;
use lazy_static::lazy_static;
use std::cell::RefCell;
use crate::utils::u64_or_bust;

#[path = "../utils.rs"] mod utils;

/// Advent of Code Day 05
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the input
    input: PathBuf,

}

fn split_stacks_and_instructions(input: &str) -> Vec<&str> {
    input.split("\n\n").collect()
}

fn parse_stacks(input: &str) -> Vec<RefCell<Vec<char>>> {
    lazy_static! {
        static ref LAST_STACKNO: Regex = Regex::new(r"(\d+)\s*$").unwrap();
    }
    let mut bottom_up = input.lines().rev();
    let stackno = LAST_STACKNO.captures(bottom_up.next().unwrap()).unwrap();
    let num_stacks = u64_or_bust(stackno.get(1).unwrap().as_str());

    let mut stacks: Vec<RefCell<Vec<char>>> = vec![RefCell::new(Vec::new()); num_stacks as usize];

    for line in bottom_up {
        let chars: Vec<char> = line.chars().collect();
        for (i, stack) in stacks.iter_mut().enumerate() {
            match chars.get(i * 4 + 1) {
                Some(c) => {
                    if c.is_ascii_alphabetic() {
                        stack.get_mut().push(*c)
                    }
                },
                None => ()
            }
        }
    }

    stacks
}

fn parse_instruction(input: &str) -> (u64, u64, u64) {
    lazy_static! {
        static ref INSTR: Regex = Regex::new(r"^move (\d+) from (\d+) to (\d+)\s*$").unwrap();
    }

    let cap = INSTR.captures(input).unwrap();
    let num = u64_or_bust(cap.get(1).unwrap().as_str());
    let from = u64_or_bust(cap.get(2).unwrap().as_str());
    let to = u64_or_bust(cap.get(3).unwrap().as_str());
    (num, from, to)
}

fn run_crane(stacks: &mut Vec<RefCell<Vec<char>>>, instructions: &str) {
    for line in instructions.lines() {
        let (num, from, to) = parse_instruction(line);
        //let fs: &mut Vec<char> = *(stacks[(from - 1) as usize].borrow_mut());
        let fs = &mut *(stacks[(from - 1) as usize].borrow_mut());
        let ts = &mut *(stacks[(to - 1) as usize].borrow_mut());
        for _ in 0..num {
            ts.push(fs.pop().unwrap());
        }
    }
}

fn run_crane_pt2(stacks: &mut Vec<RefCell<Vec<char>>>, instructions: &str) {
    for line in instructions.lines() {
        let (num, from, to) = parse_instruction(line);
        //let fs: &mut Vec<char> = *(stacks[(from - 1) as usize].borrow_mut());
        let fs = &mut *(stacks[(from - 1) as usize].borrow_mut());
        let ts = &mut *(stacks[(to - 1) as usize].borrow_mut());
        let mut split = fs.split_off(fs.len() - (num as usize) );
        ts.append(&mut split);
    }
}

fn print_tops(stacks: & Vec<RefCell<Vec<char>>>) {
    for stack in stacks {
        let s = &*stack.borrow();
        print!("{}", s[s.len() -1]);
    }
    println!("");
}

fn main() {
    let args = Args::parse();

    let mut f = File::open(args.input).expect("file not found");
    let mut input = String::new();
    f.read_to_string(&mut input).expect("something went wrong reading the file");

    let inputs = split_stacks_and_instructions(&input);
    let mut stacks = parse_stacks(inputs.get(0).unwrap());
    let mut part2_stacks = stacks.clone();
    run_crane(&mut stacks, inputs.get(1).unwrap());

    print!("part 1: ");
    print_tops(&stacks);

    run_crane_pt2(&mut part2_stacks, inputs.get(1).unwrap());

    print!("part 2: ");
    print_tops(&part2_stacks);
}