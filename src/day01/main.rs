use std::fs::File;
use std::io::prelude::*;
use clap::Parser;
use std::path::PathBuf;
use crate::utils::i64_or_bust;

#[path = "../utils.rs"] mod utils;

/// Advent of Code Day 01
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the input
    input: PathBuf,

}

fn calories_per_elf(input : &str) -> Vec<i64> {
    let mut calories : Vec<i64> = Vec::new();
    let it = input.lines();
    let mut sum : i64 = 0;
    for line in it {
        if line.is_empty() {
            if sum > 0 {
                calories.push(sum);
                sum = 0;
            }
            continue;
        }
        sum += i64_or_bust(line);
    }
    // in case there is no empty last line
    if sum > 0 {
        calories.push(sum);
    }
    calories
}

fn main() {
    let args = Args::parse();

    let mut f = File::open(args.input).expect("file not found");
    let mut input = String::new();
    f.read_to_string(&mut input).expect("something went wrong reading the file");

    println!("Most calories: {}", calories_per_elf(&input).iter().max().unwrap());
}