use std::fs::File;
use std::io::prelude::*;
use clap::Parser;
use std::path::PathBuf;
use std::collections::{HashSet, VecDeque};
use std::collections::HashMap;
use std::cmp;

#[path = "../utils.rs"] mod utils;

/// Advent of Code Day 23
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the input
    input: PathBuf,

}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West
}

fn parse_positions(input: &str) -> HashSet<(i64, i64)> {
    let mut positions: HashSet<(i64, i64)> = HashSet::new();

    for (y, line) in input.lines().enumerate() {
        //println!("{}", line);
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                //println!("inserting at ({}, {})", x, y);
                positions.insert((x as i64, y as i64));
            }
        }
    }

    positions
}

fn possible_directions(pos: (i64, i64),
                       map: &HashSet<(i64, i64)>) -> HashMap<Direction, (i64, i64)> {
    let mut directions = HashMap::new();
    if !map.contains(&(pos.0 - 1, pos.1 - 1)) &&
        !map.contains(&(pos.0, pos.1 - 1)) &&
        !map.contains(&(pos.0 + 1, pos.1 - 1)) {
        directions.insert(Direction::North, (pos.0, pos.1 - 1));
    }
    if !map.contains(&(pos.0 + 1, pos.1 - 1)) &&
        !map.contains(&(pos.0 + 1, pos.1)) &&
        !map.contains(&(pos.0 + 1, pos.1 + 1)) {
        directions.insert(Direction::East, (pos.0 + 1, pos.1));
    }
    if !map.contains(&(pos.0 - 1, pos.1 + 1)) &&
        !map.contains(&(pos.0, pos.1 + 1)) &&
        !map.contains(&(pos.0 + 1, pos.1 + 1)) {
        directions.insert(Direction::South, (pos.0, pos.1 + 1));
    }
    if !map.contains(&(pos.0 - 1, pos.1 - 1)) &&
        !map.contains(&(pos.0 - 1, pos.1)) &&
        !map.contains(&(pos.0 - 1, pos.1 + 1)) {
        directions.insert(Direction::West, (pos.0 - 1, pos.1));
    }
    directions
}

fn decide_move(pos: (i64, i64),
               map: &HashSet<(i64, i64)>,
               priority: &VecDeque<Direction>) -> (i64, i64) {
    let possible_moves = possible_directions(pos, map);
    if possible_moves.len() == 4 || possible_moves.len() == 0 {
        return pos;
    }
    for dir in priority {
        if let Some(mv) = possible_moves.get(dir) {
            return *mv;
        }
    }
    // this is be redundant, but whatever
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
    let some_point = map.iter().next().unwrap();
    let mut min_x: i64 = some_point.0;
    let mut max_x: i64 = some_point.1;
    let mut min_y: i64 = some_point.0;
    let mut max_y: i64 = some_point.1;

    for elf in map {
        min_x = cmp::min(min_x, elf.0);
        min_y = cmp::min(min_y, elf.1);
        max_x = cmp::max(max_x, elf.0);
        max_y = cmp::max(max_y, elf.1);
    }

    ((min_x, min_y), (max_x, max_y))
}

fn print_state(map: &HashSet<(i64, i64)>) {
    let field = bounding_box(&map);
    println!("bounding box: {:?}", field);
    for line in field.0.1..(field.1.1 + 1) {
        for row in field.0.0..(field.1.0 + 1) {
            let c = if map.contains(&(row, line)) { '#' } else { '.' };
            print!("{}", c);
        }
        println!();
    }
    println!();
}

fn part1(initial_state: &HashSet<(i64, i64)>) -> i64 {
    let mut dir_prios = VecDeque::from([Direction::North, Direction::South, Direction::West, Direction::East]);
    let mut state = initial_state.clone();
    //print_state(&state);
    for _ in 0..10 {
        //println!("directions priority: {:?}", dir_prios);
        state = execute_moves(&plan_moves(&state, &dir_prios));
        //print_state(&state);
        let old_prio1 = dir_prios.pop_front().unwrap();
        dir_prios.push_back(old_prio1);
    }

    let field = bounding_box(&state);
    //println!("bounding box: {:?}", field);
    let x_size = (field.1.0 - field.0.0) + 1;
    let y_size = (field.1.1 - field.0.1) + 1;
    //println!("x size: {}, y size: {}", x_size, y_size);
    let field_size = x_size * y_size;
    let num_elves = state.len() as i64;
    //println!("field size: {}, elves: {}", field_size, num_elves);
    let empty_tiles = field_size - num_elves;

    empty_tiles
}

fn part2(initial_state: &HashSet<(i64, i64)>) -> i64 {
    let mut dir_prios = VecDeque::from([Direction::North, Direction::South, Direction::West, Direction::East]);
    let mut state = initial_state.clone();
    let mut moves = 1;

    loop {
        let planned_moves = &plan_moves(&state, &dir_prios);
        let mut no_elf_moves = true;
        for (to, from) in planned_moves {
            if from.len() > 1 {
                continue
            }
            if from[0] != *to {
                no_elf_moves = false;
                break;
            }
        }
        if no_elf_moves {
            break;
        }
        state = execute_moves(planned_moves);
        moves += 1;
        let old_prio1 = dir_prios.pop_front().unwrap();
        dir_prios.push_back(old_prio1);
    }

    moves
}

fn main() {
    let args = Args::parse();

    let mut f = File::open(args.input).expect("file not found");
    let mut input = String::new();
    f.read_to_string(&mut input).expect("something went wrong reading the file");

    let positions = parse_positions(&input);

    println!("part 1: {}", part1(&positions));
    println!("part 2: {}", part2(&positions));
}