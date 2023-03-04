use days::day24::puzzle::ssa::Val;
use fs::File;
use itertools::Itertools;
use std::io::prelude::*;
use std::{fs, str};

use days::day24;

fn main() {
    use day24::puzzle::{emulate_sections, ssa, tree, ALU};

    let bytes = fs::read("day24.txt").unwrap();
    let s = str::from_utf8(&bytes).unwrap();

    let (_, instrs) = day24::puzzle::parser::instructions(&s).unwrap();
    let params = day24::puzzle::parser::extract_params(&s);
    println!("instrs.len() = {}", instrs.len());
    let (ssa, state) = ssa::from_straightline(&instrs);
    let (eliminated, bindings) = ssa::eliminate_constants(&ssa, state);
    println!("eliminated.len() = {}", eliminated.len());

    let digits = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5];
    let mut alu = ALU::new();
    alu.run(&instrs, digits.clone());
    let alu_res = alu.z;
    let emu_res = emulate_sections(&digits, &params);

    println!("ALU {} EMU {}", alu_res, emu_res);
    println!("params.len() = {}", params.len());
    for param in params {
        println!("{:?}", param)
    }

    //let tree = tree::from_eliminated_ssa(&ssa, &ssa::Bindings::new());
    let mut dot = File::create("day24.dot").unwrap();
    /*
    dot.write(b"digraph {\n").unwrap();
    dot.write(tree::print_tree(&tree).as_bytes()).unwrap();
    dot.write(b"}").unwrap();
    */
    let empty_bindings = ssa::Bindings::new();
    let mut dot = File::create("day24.dot").unwrap();
    dot.write(b"digraph {\n").unwrap();
    for instr in ssa {
        //println!("{:?}", instr);

        dot.write(instr.as_dot(&empty_bindings).as_bytes()).unwrap();
        dot.write(b"\n").unwrap();
    }
    dot.write(b"}").unwrap();
}
