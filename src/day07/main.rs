use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use clap::Parser;
use std::path::PathBuf;
use regex::Regex;
use lazy_static::lazy_static;
use itertools::Itertools;
use crate::utils::u64_or_bust;

#[path = "../utils.rs"] mod utils;

/// Advent of Code Day 07
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the input
    input: PathBuf,

}

#[derive(Debug, PartialEq)]
enum CMD {
    LS,
    CD
}


#[derive(Debug, PartialEq)]
enum FsObj {
    Dir,
    File(u64)
}


fn parse_command(input: &str) -> (Option<CMD>, Option<&str>) {
    lazy_static! {
        static ref CMD_REGEX: Regex = Regex::new(r"^\$ (\w+) ?(.*)$").unwrap();
    }

    let caps = CMD_REGEX.captures(input).unwrap();
    let cmdstr = caps.get(1).unwrap().as_str();
    let args = match caps.get(2) {
        Some(i) => Some(i.as_str()),
        None => None
    };
    
    let cmd = match cmdstr {
        "cd" => Some(CMD::CD),
        "ls" => Some(CMD::LS),
        _ => None,
    };

    (cmd, args)
}

fn parse_ls_line(input: &str) -> (FsObj, &str) {
    lazy_static! {
        static ref LS_REGEX: Regex = Regex::new(r"^(.+) (.+)$").unwrap();
    }
    let caps = LS_REGEX.captures(input).unwrap();
    let size_or_dir = caps.get(1).unwrap().as_str();
    let name = caps.get(2).unwrap().as_str();
    if size_or_dir == "dir" {
        (FsObj::Dir, name)
    } else {
        let size = u64_or_bust(size_or_dir);
        (FsObj::File(size), name)
    }
}


fn parse_input(input: &str) {
    let mut cur_dir = PathBuf::new();
    let mut sizes: HashMap<PathBuf, u64> = HashMap::new();

    let mut lines = input.lines().peekable();
    while let Some(i) = lines.next() {
        if i.starts_with('$') {
            let (c, args) = parse_command(i);
            let cmd = c.unwrap();
            println!("is command: {:?} {}", cmd, args.unwrap());
            match cmd {
                CMD::CD => {
                    let path = args.unwrap();
                    if path == ".." {
                        cur_dir.pop();
                    } else {
                        cur_dir.push(path);
                        if !sizes.contains_key(&cur_dir) {
                            sizes.insert(cur_dir.clone(), 0);
                        }
                    }
                    println!("cur_dir: {}", cur_dir.to_str().unwrap());
                },
                CMD::LS => {
                    while let Some(next) = lines.peek() {
                        if next.starts_with('$') {
                            break;
                        } else {
                            let content = lines.next().unwrap();
                            let (fstype, name) = parse_ls_line(content);
                            if let FsObj::File(size) = fstype {
                                *sizes.get_mut(&cur_dir).unwrap() += size;
                            }
                            println!("content: {:?}, {}", fstype, name);
                        }
                    }
                }
            }
        } else {
            println!("oh-oh, this should not happen: {}", i);
        }
    }


    // there are surely better ways to do this, but I'm lazy
    let sizes_clone = sizes.clone();
    for dir in sizes_clone.keys().sorted().rev() {
        if let Some(parent) = dir.parent() {
            *sizes.get_mut(parent).unwrap() += sizes[dir];
        } else {
            println!("{} has no parent", dir.to_str().unwrap());
        }
    }
    
    println!("------------");
    println!("dir sizes:");
    for dir in sizes.keys().sorted() {
        println!("{}: {}", dir.to_str().unwrap(), sizes[dir])
    }
}

fn main() {
    let args = Args::parse();

    let mut f = File::open(args.input).expect("file not found");
    let mut input = String::new();
    f.read_to_string(&mut input).expect("something went wrong reading the file");

    parse_input(&input);
}