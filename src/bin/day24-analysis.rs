use fs::File;
use std::io::prelude::*;
use std::{fs, str};

use days::day24;

fn main() {
    use day24::puzzle::ssa;

    let bytes = fs::read("day24.txt").unwrap();
    let s = str::from_utf8(&bytes).unwrap();

    let (_, instrs) = day24::puzzle::parser::instructions(s).unwrap();
    println!("instrs.len() = {}", instrs.len());
    let (ssa, state) = ssa::from_straightline(instrs);
    let (eliminated, bindings) = ssa::eliminate_constants(ssa, state);

    let mut dot = File::create("day24.dot").unwrap();
    dot.write(b"digraph {\n").unwrap();
    for instr in eliminated {
        println!("{:?}", instr);

        dot.write(instr.as_dot(&bindings).as_bytes()).unwrap();
        dot.write(b"\n").unwrap();
    }
    dot.write(b"}").unwrap();
}
