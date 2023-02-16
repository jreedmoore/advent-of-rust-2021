#![feature(hash_drain_filter)]
mod days;
mod util;
use days::*;

use std::env;
use std::time::Instant;
use std::{fs, str};

struct Program<'a> {
    pub name: &'a str,
    pub input: &'a str,
    pub entry: fn(&'a str) -> Option<u64>,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let programs: Vec<Program> = vec![
        Program {
            name: "day11-1",
            input: "day11.txt",
            entry: day11::part_one,
        },
        Program {
            name: "day11-2",
            input: "day11.txt",
            entry: day11::part_two,
        },
        Program {
            name: "day12-1",
            input: "day12.txt",
            entry: day12::part_one,
        },
        Program {
            name: "day12-2",
            input: "day12.txt",
            entry: day12::part_two,
        },
        Program {
            name: "day13-1",
            input: "day13.txt",
            entry: day13::part_one,
        },
        Program {
            name: "day13-2",
            input: "day13.txt",
            entry: day13::part_two,
        },
        Program {
            name: "day14-1",
            input: "day14.txt",
            entry: day14::part_one,
        },
        Program {
            name: "day14-2",
            input: "day14.txt",
            entry: day14::part_two,
        },
        Program {
            name: "day15-1",
            input: "day15.txt",
            entry: day15::part_one,
        },
        Program {
            name: "day15-2",
            input: "day15.txt",
            entry: day15::part_two,
        },
        Program {
            name: "day16-1",
            input: "day16.txt",
            entry: day16::part_one,
        },
        Program {
            name: "day16-2",
            input: "day16.txt",
            entry: day16::part_two,
        },
        Program {
            name: "day17-1",
            input: "day17.txt",
            entry: day17::part_one,
        },
        Program {
            name: "day17-2",
            input: "day17.txt",
            entry: day17::part_two,
        },
        Program {
            name: "day18-1",
            input: "day18.txt",
            entry: day18::part_one,
        },
        Program {
            name: "day18-2",
            input: "day18.txt",
            entry: day18::part_two,
        },
        Program {
            name: "day19-1",
            input: "day19.txt",
            entry: day19::part_one,
        },
        Program {
            name: "day19-2",
            input: "day19.txt",
            entry: day19::part_two,
        },
        Program {
            name: "day20-1",
            input: "day20.txt",
            entry: day20::part_one,
        },
        Program {
            name: "day20-2",
            input: "day20.txt",
            entry: day20::part_two,
        },
        Program {
            name: "day21-1",
            input: "day21.txt",
            entry: day21::part_one,
        },
        Program {
            name: "day21-2",
            input: "day21.txt",
            entry: day21::part_two,
        },
    ];

    let program = programs.iter().find(|prog| prog.name == args[1]).unwrap();
    let bytes = fs::read(program.input).unwrap();
    let s = str::from_utf8(&bytes).unwrap();
    let f = program.entry;
    let start = Instant::now();
    let answer = f(s);
    let end = Instant::now();
    let time = end.duration_since(start);
    println!("{}: {:?}\ntook: {:?}", program.name, answer, time);
}
