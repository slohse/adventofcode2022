use std::fs::File;
use std::io::prelude::*;
use clap::Parser;
use std::path::PathBuf;
use std::collections::{HashSet, VecDeque};
use std::collections::HashMap;
use std::cmp;
use crate::utils::i64_or_bust;

#[path = "../utils.rs"] mod utils;

/// Advent of Code Day 23
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the input
    input: PathBuf,

}

enum Direction {
    North,
    East,
    South,
    West
}

fn parse_positions(input: &str) -> HashSet<(i64, i64)> {
    let mut positions: HashSet<(i64, i64)> = HashSet::new();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                positions.insert((x as i64, y as i64));
            }
        }
    }

    positions
}

fn new_pos_if_clear(pos: (i64, i64),
                    dir: &Direction,
                    map: &HashSet<(i64, i64)>) -> Option<(i64, i64)> {
    match dir {
        Direction::North => {
            if !map.contains(&(pos.0 - 1, pos.1 - 1)) &&
                !map.contains(&(pos.0, pos.1 - 1)) &&
                !map.contains(&(pos.0 + 1, pos.1 - 1)) {
                    return Some((pos.0, pos.1 - 1));
            }
        },
        Direction::East => {
            if !map.contains(&(pos.0 + 1, pos.1 - 1)) &&
                !map.contains(&(pos.0 + 1, pos.1)) &&
                !map.contains(&(pos.0 + 1, pos.1 + 1)) {
                    return Some((pos.0 + 1, pos.1));
            }
        },
        Direction::South => {
            if !map.contains(&(pos.0 - 1, pos.1 + 1)) &&
                !map.contains(&(pos.0, pos.1 + 1)) &&
                !map.contains(&(pos.0 + 1, pos.1 + 1)) {
                    return Some((pos.0, pos.1 + 1));
            }
        },
        Direction::West => {
            if !map.contains(&(pos.0 - 1, pos.1 - 1)) &&
                !map.contains(&(pos.0 - 1, pos.1)) &&
                !map.contains(&(pos.0 - 1, pos.1 + 1)) {
                    return Some((pos.0 - 1, pos.1));
            }
        },
    }
    None
}

fn decide_move(pos: (i64, i64),
               map: &HashSet<(i64, i64)>,
               priority: &VecDeque<Direction>) -> (i64, i64) {
    for dir in priority {
        if let Some(new_pos) = new_pos_if_clear(pos, dir, map) {
            return new_pos;
        }
    }
    return pos;
}

fn plan_moves(map: &HashSet<(i64, i64)>,
              priority: &VecDeque<Direction>) -> HashMap<(i64, i64), Vec<(i64, i64)>> {
    let mut planned_moves: HashMap<(i64, i64), Vec<(i64, i64)>> = HashMap::new();
    for elf in map {
        let mv = decide_move(*elf, map, priority);
        if let Some(l) = planned_moves.get_mut(&mv) {
            l.push(*elf)
        } else {
            let mut l = Vec::new();
            l.push(*elf);
            planned_moves.insert(mv, l);
        }
    }

    planned_moves
}

fn execute_moves(moves: &HashMap<(i64, i64), Vec<(i64, i64)>>) -> HashSet<(i64, i64)> {
    let mut new_map = HashSet::new();
    for (pos, sources) in moves {
        if sources.len() == 1 {
            new_map.insert(*pos);
        } else {
            for elf in sources {
                new_map.insert(*elf);
            }
        }
    }

    new_map
}

fn bounding_box(map: &HashSet<(i64, i64)>) -> ((i64, i64), (i64, i64)) {
    let mut min_x: i64 = 0;
    let mut max_x: i64 = 0;
    let mut min_y: i64 = 0;
    let mut max_y: i64 = 0;

    for elf in map {
        min_x = cmp::min(min_x, elf.0);
        min_y = cmp::min(min_y, elf.1);
        max_x = cmp::max(max_x, elf.0);
        max_y = cmp::max(max_y, elf.1);
    }

    ((min_x, min_y), (max_x, max_y))
}

fn part1(initial_state: &HashSet<(i64, i64)>) -> i64 {
    let mut dir_prios = VecDeque::from([Direction::North, Direction::South, Direction::West, Direction::East]);
    let mut state = initial_state.clone();
    for _ in 1..10 {
        state = execute_moves(&plan_moves(&state, &dir_prios));
        let old_prio1 = dir_prios.pop_front().unwrap();
        dir_prios.push_back(old_prio1);
    }

    let field = bounding_box(&state);
    let x_size = field.1.0 - field.0.0;
    let y_size = field.1.1 - field.0.1;
    let field_size = x_size * y_size;
    let empty_tiles = field_size - state.len() as i64;

    empty_tiles
}

fn main() {
    let args = Args::parse();

    let mut f = File::open(args.input).expect("file not found");
    let mut input = String::new();
    f.read_to_string(&mut input).expect("something went wrong reading the file");

    let positions = parse_positions(&input);

    println!("part 1: {}", part1(&positions));
}