use std::fs::File;
use std::io::prelude::*;
use clap::Parser;
use std::path::PathBuf;
use regex::Regex;
use lazy_static::lazy_static;
use crate::utils::i64_or_bust;

#[path = "../utils.rs"] mod utils;

enum CMD {
    NOOP,
    ADDX(i64),
}

/// Advent of Code Day 10
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the input
    input: PathBuf,

}

fn parse_cmd(input: &str) -> CMD {
    lazy_static! {
        static ref INSTR: Regex = Regex::new(r"^(\w+) ?(-?\d+)?$").unwrap();
    }
    let cap = INSTR.captures(input).unwrap();

    match cap.get(1).unwrap().as_str() {
        "noop" => CMD::NOOP,
        "addx" => {
            let num = i64_or_bust(cap.get(2).unwrap().as_str());
            CMD::ADDX(num)
        },
        i => panic!("unknown instruction {}", i)
    }
}

fn is_snapshot(cycle: usize, first: usize, frequency: usize) -> bool {
    cycle == first || (cycle > first && ((cycle - first) % frequency == 0))
}

fn run(input: &str, first_snapshot: usize, snapshot_frequency: usize) -> Vec<(usize, i64)> {
    let mut xreg: i64 = 1;
    let mut cycle: usize = 1;
    let mut snapshots = Vec::new();

    for line in input.lines() {
        if is_snapshot(cycle, first_snapshot, snapshot_frequency) {
            snapshots.push((cycle, xreg));
        }
        let cmd = parse_cmd(line);
        match cmd {
            CMD::NOOP => {
                cycle += 1
            }
            CMD::ADDX(i) => {
                cycle += 1;
                if is_snapshot(cycle, first_snapshot, snapshot_frequency) {
                    snapshots.push((cycle, xreg));
                }
                xreg += i;
                cycle += 1;
            }
        }
    }
    snapshots
}

fn part1(input: &str) -> i64 {
    let snapshots = run(input, 20, 40);
    println!("{:?}", snapshots);
    let mut sum: i64 = 0;
    for (cycle, value) in snapshots {
        sum += (cycle as i64) * value;
    }

    sum
}


fn main() {
    let args = Args::parse();

    let mut f = File::open(args.input).expect("file not found");
    let mut input = String::new();
    f.read_to_string(&mut input).expect("something went wrong reading the file");

    println!("part 1: {}", part1(&input));

}