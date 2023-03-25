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

    let mut d = 13621111481315;
    let mut digits = vec![];
    while d != 0 {
        digits.push(d % 10);
        d = d / 10;
    }
    digits.reverse();
    //let digits = vec![5,9,9,9,8,4,2,6,9,9,7,9,7,9];
    let mut alu = ALU::new();
    alu.run(&instrs, digits.clone());
    let alu_res = alu.z;
    let emu_res = emulate_sections(&digits, &params);

    println!("ALU {} EMU {}", alu_res, emu_res);
    println!("params.len() = {}", params.len());

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
