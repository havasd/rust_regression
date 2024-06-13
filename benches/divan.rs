use day9::{
    rust::{p1, p2},
    swift::{day9p1_rust_bridge, day9p2_rust_bridge},
};
use swift_rs::{self, SRString};

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part1() {
    p1(divan::black_box(include_str!("../input.txt",)));
}

#[divan::bench]
fn part2() {
    p2(divan::black_box(include_str!("../input.txt",)));
}

#[divan::bench]
fn part1_swift() {
    unsafe {
        day9p1_rust_bridge(divan::black_box(SRString::from(include_str!(
            "../input.txt",
        ))));
    }
}

#[divan::bench]
fn part2_swift() {
    unsafe {
        day9p2_rust_bridge(divan::black_box(SRString::from(include_str!(
            "../input.txt",
        ))));
    }
}
