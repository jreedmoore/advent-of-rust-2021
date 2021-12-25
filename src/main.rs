#![feature(hash_drain_filter)]
mod days;
use days::*;

use std::{fs, str};
use std::env;

struct Program<'a> {
    pub name: &'a str,
    pub input: &'a str,
    pub entry: fn(&'a str) -> Option<u32>
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let programs : Vec<Program> = vec![
        Program{name: "day11-1", input: "day11.txt", entry: day11::part_one}
    ,   Program{name: "day11-2", input: "day11.txt", entry: day11::part_two}
    ,   Program{name: "day12-1", input: "day12.txt", entry: day12::part_one}
    ,   Program{name: "day12-2", input: "day12.txt", entry: day12::part_two}
    ,   Program{name: "day13-1", input: "day13.txt", entry: day13::part_one}
    ,   Program{name: "day13-2", input: "day13.txt", entry: day13::part_two}
    ,   Program{name: "day14-1", input: "day14.txt", entry: day14::part_one}
    ];

    let program = programs.iter().find(|prog| prog.name == args[1]).unwrap();
    let bytes = fs::read(program.input).unwrap();
    let s = str::from_utf8(&bytes).unwrap();
    let f = program.entry;
    println!("{}: {}", program.name, f(s).unwrap());
}