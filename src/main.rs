mod days;
use days::*;

use std::{fs, str};


fn main() {
    let bytes = fs::read("day11.txt").unwrap();
    let s = str::from_utf8(&bytes).unwrap();
    println!("{}", day11::part_two(s).unwrap());
}