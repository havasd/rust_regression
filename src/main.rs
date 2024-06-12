use std::{
    fs::File,
    io::Read,
};

use day9::{p1, p2};
use day9::swift::day9p1ForRust;
use day9::swift::day9p2ForRust;
use swift_rs::{self, SRString};

fn main() {
    let mut f = File::open("input.txt").expect("can't open file");
    let mut buf = String::new();
    f.read_to_string(&mut buf).expect("can't read file");
    let result = p1(&buf);
    println!("{result}");
    let result = p2(&buf);
    println!("{result}");
    let result = unsafe {day9p1ForRust(SRString::from(buf.as_str()))};
    println!("{result}");
    let result = unsafe {day9p2ForRust(SRString::from(buf.as_str()))};
    println!("{result}");
}

