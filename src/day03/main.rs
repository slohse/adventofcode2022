#![feature(iter_next_chunk)]

use std::fs::File;
use std::io::prelude::*;
use clap::Parser;
use std::path::PathBuf;
use std::collections::HashSet;

/// Advent of Code Day 03
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the input
    input: PathBuf,

}


fn find_wrong_item(rucksack: &str) -> Option<char> {
    let volume = rucksack.len();
    let left = &rucksack[0..volume/2];
    let right = &rucksack[volume/2..volume];

    for item in left.chars() {
        if right.contains(item) {
            return Some(item);
        }
    }
    None
}

fn item_priority(item: char) -> i64 {
    if item.is_ascii_lowercase() {
        (item as i64) - 96
    } else if item.is_ascii_uppercase() {
        (item as i64) - 38
    } else {
        0
    }
}

fn common_item(items: &[&str]) -> char {
    let one: HashSet<char> = items[0].chars().collect();
    let two: HashSet<char> = items[1].chars().collect();
    let three: HashSet<char> = items[2].chars().collect();

    let one_two: HashSet<char> = one.intersection(&two).cloned().collect();
    let all: Vec<&char> = one_two.intersection(&three).collect();

    **all.get(0).unwrap()
}

fn part1(input: &str) -> i64 {
    input.lines().fold(0, |acc, x| acc + item_priority(find_wrong_item(x).unwrap()))
}

fn part2(input: &str) -> i64 {
    let mut iter = input.lines();

    let mut sum = 0;
    while let Ok(i) = iter.next_chunk::<3>() {
        sum += item_priority(common_item(&i));
    }
    sum
}


fn main() {
    let args = Args::parse();

    let mut f = File::open(args.input).expect("file not found");
    let mut input = String::new();
    f.read_to_string(&mut input).expect("something went wrong reading the file");

    println!("part 1: {}", part1(&input));
    println!("part 2: {}", part2(&input));
}