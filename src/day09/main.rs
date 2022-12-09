use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;
use clap::Parser;
use std::path::PathBuf;
use regex::Regex;
use lazy_static::lazy_static;
use std::cmp::Ordering;
use crate::utils::i64_or_bust;

#[path = "../utils.rs"] mod utils;

/// Advent of Code Day 09
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the input
    input: PathBuf,

}

fn calculate_tail_move(head: (i64, i64), tail: (i64, i64)) -> (i64, i64) {
    let x_diff = head.0 - tail.0;
    let y_diff = head.1 - tail.1;

    let x_dir = x_diff.cmp(&0);
    let y_dir = y_diff.cmp(&0);

    if x_diff.abs() > 1 || y_diff.abs() > 1 {
        match y_dir {
            Ordering::Less => {
                match x_dir {
                    Ordering::Less => (tail.0 - 1, tail.1 - 1),
                    Ordering::Equal => (tail.0, tail.1 - 1),
                    Ordering::Greater => (tail.0 + 1, tail.1 - 1)
                }
            },
            Ordering::Equal => {
                match x_dir {
                    Ordering::Less => (tail.0 - 1, tail.1),
                    Ordering::Equal => (tail.0, tail.1), // this case should not occur due to the abs() clauses
                    Ordering::Greater => (tail.0 + 1, tail.1)
                }
            },
            Ordering::Greater => {
                match x_dir {
                    Ordering::Less => (tail.0 - 1, tail.1 + 1),
                    Ordering::Equal => (tail.0, tail.1 + 1),
                    Ordering::Greater => (tail.0 + 1, tail.1 + 1)
                }
            }
        }
    } else {
        tail
    }
}

fn calculate_head_move(dir: &str, head: (i64, i64)) -> (i64, i64) {
    match dir {
        "U" => (head.0, head.1 + 1),
        "D" => (head.0, head.1 - 1),
        "L" => (head.0 - 1, head.1),
        "R" => (head.0 + 1, head.1),
        c => panic!("Unknown direction: {}", c)
    }
}

fn part1(input: &str) -> usize {
    let mut head: (i64, i64) = (0, 0);
    let mut tail = head.clone();
    lazy_static! {
        static ref INSTR: Regex = Regex::new(r"^([UDLR]) (\d+)$").unwrap();
    }
    let mut tailspots: HashSet<(i64, i64)> = HashSet::new();

    for line in input.lines() {
        let cap = INSTR.captures(line).unwrap();
        let dir = cap.get(1).unwrap().as_str();
        let num = i64_or_bust(cap.get(2).unwrap().as_str());

        for _ in 0..num {
            let new_head = calculate_head_move(dir, head);
            let new_tail = calculate_tail_move(new_head, tail);
            //println!("head: ({}, {}) -> ({}, {})", head.0, head.1, new_head.0, new_head.1);
            //println!("tail: ({}, {}) -> ({}, {})", tail.0, tail.1, new_tail.0, new_tail.1);
            tailspots.insert(new_tail);
            head = new_head;
            tail = new_tail;
        }
    }
    tailspots.len()
}

fn part2(input: &str) -> usize {
    let mut head: (i64, i64) = (0, 0);
    let mut tail: Vec<(i64, i64)> = Vec::new();
    for _ in 0..9 {
        tail.push((0, 0))
    }
    lazy_static! {
        static ref INSTR: Regex = Regex::new(r"^([UDLR]) (\d+)$").unwrap();
    }
    let mut tailspots: HashSet<(i64, i64)> = HashSet::new();

    for line in input.lines() {
        let cap = INSTR.captures(line).unwrap();
        let dir = cap.get(1).unwrap().as_str();
        let num = i64_or_bust(cap.get(2).unwrap().as_str());

        for _ in 0..num {
            head = calculate_head_move(dir, head);
            let mut cur_knot = head;
            for knot in tail.iter_mut() {
                *knot = calculate_tail_move(cur_knot, *knot);
                cur_knot = *knot;
            }
            //println!("head: ({}, {}) -> ({}, {})", head.0, head.1, new_head.0, new_head.1);
            //println!("tail: ({}, {}) -> ({}, {})", tail.0, tail.1, new_tail.0, new_tail.1);
            tailspots.insert(cur_knot);
        }
    }
    tailspots.len()
}

fn main() {
    let args = Args::parse();

    let mut f = File::open(args.input).expect("file not found");
    let mut input = String::new();
    f.read_to_string(&mut input).expect("something went wrong reading the file");

    println!("part 1: {}", part1(&input));
    println!("part 2: {}", part2(&input));
}