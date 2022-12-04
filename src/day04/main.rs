use std::fs::File;
use std::io::prelude::*;
use clap::Parser;
use std::path::PathBuf;
use regex::Regex;
use lazy_static::lazy_static;
use crate::utils::u64_or_bust;

#[path = "../utils.rs"] mod utils;

/// Advent of Code Day 04
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the input
    input: PathBuf,

}

fn parse_input(input: &str) -> Vec<((u64,u64),(u64,u64))> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(\d+)-(\d+),(\d+)-(\d+)").unwrap();
    }
    let mut pairs = Vec::new();
    println!("input:");
    print!("{}", input);
    println!("-----------");
    for cap in RE.captures_iter(input) {
        pairs.push(((u64_or_bust(&cap[1]),
                     u64_or_bust(&cap[2])),
                    (u64_or_bust(&cap[3]),
                     u64_or_bust(&cap[4]))));
    }

    pairs
}

fn is_fully_contained(pair: ((u64,u64),(u64,u64))) -> bool {
    let result = (pair.0.0 <= pair.1.0 && pair.0.1 >= pair.1.1) || (pair.1.0 <= pair.0.0 && pair.1.1 >= pair.0.1);
    println!("({p00} <= {p10} && {p01} <= {p11}) || ({p10} <= {p00} && {p11} <= {p01}) => {r}",
             p00 = pair.0.0,
             p01 = pair.0.1,
             p10 = pair.1.0,
             p11 = pair.1.1,
             r = result);

    result
}

fn part1(pairs: &Vec<((u64,u64),(u64,u64))>) -> u64 {
    let mut count = 0;
    for pair in pairs {
        if is_fully_contained(*pair) {
            count+= 1;
        }
    }

    count
}

fn main() {
    let args = Args::parse();

    let mut f = File::open(args.input).expect("file not found");
    let mut input = String::new();
    f.read_to_string(&mut input).expect("something went wrong reading the file");

    let re = Regex::new(r"^(\d+)-(\d+),(\d+)-(\d+)").unwrap();
    assert!(re.is_match("2-4,6-8"));

    let pairs = parse_input(&input);

    println!("part 1: {}", part1(&pairs));

}