use iai_callgrind::{
    library_benchmark, library_benchmark_group, main
};

use day9_rust::{p1, p2};
use std::fs::File;
use std::io::Read;

use std::hint::black_box;

#[library_benchmark]
fn bench_p1() {
    let data = {
                let mut f = File::open("input.txt").expect("can't open file");
                let mut buf = String::new();
                f.read_to_string(&mut buf).expect("can't read file");
                buf
            };
    black_box(p1(&data));
}

#[library_benchmark]
fn bench_p2() {
    let data =  {
                let mut f = File::open("input.txt").expect("can't open file");
                let mut buf = String::new();
                f.read_to_string(&mut buf).expect("can't read file");
                buf
            };
    black_box(p2(&data));
}

library_benchmark_group!(
    name = bench_group;
    benchmarks =
        bench_p1,
        bench_p2,
);

main!(library_benchmark_groups = bench_group);
