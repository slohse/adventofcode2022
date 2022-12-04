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

fn parse_line(input: &str) -> ((u64,u64),(u64,u64)) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(\d+)-(\d+),(\d+)-(\d+)").unwrap();
    }
    let caps = RE.captures(input).unwrap();
    ((u64_or_bust(caps.get(1).unwrap().as_str()),
      u64_or_bust(caps.get(2).unwrap().as_str())),
     (u64_or_bust(caps.get(3).unwrap().as_str()),
      u64_or_bust(caps.get(4).unwrap().as_str())))
}

fn is_fully_contained(pair: ((u64,u64),(u64,u64))) -> bool {
    (pair.0.0 <= pair.1.0 && pair.0.1 >= pair.1.1) || (pair.1.0 <= pair.0.0 && pair.1.1 >= pair.0.1)
}

fn part1(input: &str) -> u64 {
    let mut count = 0;
    for line in input.lines() {
        if is_fully_contained(parse_line(line)) {
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

    println!("part 1: {}", part1(&input));

}