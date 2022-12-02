use std::fs::File;
use std::io::prelude::*;
use clap::Parser;
use std::path::PathBuf;

/// Advent of Code Day 02
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the input
    input: PathBuf,

}

#[derive(PartialEq, Clone, Copy)]
enum RPS {
    Rock,
    Paper,
    Scissors
}

#[derive(PartialEq, Clone, Copy)]
enum Outcome {
    Lose,
    Draw,
    Win
}


fn decrypt(input: char) -> Option<RPS> {
    match input {
        'A' | 'X' => Some(RPS::Rock),
        'B' | 'Y' => Some(RPS::Paper),
        'C' | 'Z' => Some(RPS::Scissors),
        _ => {
            println!("decrypt got '{}'", input);
            None
        }
    }
}

fn decrypt_outcome(input: char) -> Option<Outcome> {
    match input {
        'X' => Some(Outcome::Lose),
        'Y' => Some(Outcome::Draw),
        'Z' => Some(Outcome::Win),
        _ => {
            println!("decrypt_outcome got '{}'", input);
            None
        }
    }
}

fn match_outcome(opponent: RPS, outcome: Outcome) -> RPS {
    match opponent {
        RPS::Rock => {
            match outcome {
                Outcome::Lose => RPS::Scissors,
                Outcome::Draw => RPS::Rock,
                Outcome::Win => RPS::Paper,
            }
        },
        RPS::Paper => {
            match outcome {
                Outcome::Lose => RPS::Rock,
                Outcome::Draw => RPS::Paper,
                Outcome::Win => RPS::Scissors,
            }
        },
        RPS::Scissors => {
            match outcome {
                Outcome::Lose => RPS::Paper,
                Outcome::Draw => RPS::Scissors,
                Outcome::Win => RPS::Rock,
            }
        },        
    }
}

fn matchscore(opponent: RPS, me: RPS) -> i64 {
    if opponent == me {
        3
    } else {
        match opponent {
            RPS::Rock => if me == RPS::Paper { 6 } else { 0 },
            RPS::Paper => if me == RPS::Scissors { 6 } else { 0 },
            RPS::Scissors => if me == RPS::Rock { 6 } else { 0 },
        }
    }
}

fn shapescore(shape: RPS) -> i64 {
    match shape {
        RPS::Rock => 1,
        RPS::Paper => 2,
        RPS::Scissors => 3,
    }
}

fn roundscore(input: &str) -> i64 {
    if input.len() >= 3 {
        let mut c = input.chars();
        let opponent = decrypt(c.nth(0).unwrap()).unwrap();
        let me = decrypt(c.nth(1).unwrap()).unwrap();
        matchscore(opponent, me) + shapescore(me)
    } else {
        0
    }
}

fn roundscore2(input: &str) -> i64 {
    if input.len() >= 3 {
        let mut c = input.chars();
        let opponent = decrypt(c.nth(0).unwrap()).unwrap();
        let outcome = decrypt_outcome(c.nth(1).unwrap()).unwrap();
        let me = match_outcome(opponent, outcome);
        matchscore(opponent, me) + shapescore(me)
    } else {
        0
    }
}

fn tournamentscore(input: &str, f: &dyn Fn(&str) -> i64) -> i64 {
    input.lines().fold(0, |acc, x| acc + f(x))
}


fn main() {
    let args = Args::parse();

    let mut f = File::open(args.input).expect("file not found");
    let mut input = String::new();
    f.read_to_string(&mut input).expect("something went wrong reading the file");

    println!("Tournament score: {}", tournamentscore(&input, &roundscore));
    println!("Part 2: {}", tournamentscore(&input, &roundscore2));

}