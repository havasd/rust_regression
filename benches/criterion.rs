// a criterion benchmark for p2, p2_reverse, and p2_maps

use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use day9::{p1, p2, swift::day9p1ForRust, swift::day9p2ForRust};
use std::fs::File;
use std::io::Read;
use swift_rs::{self, SRString};

fn bench_p1(c: &mut Criterion) {
    let mut g = c.benchmark_group("criterion");
    g.bench_function("p1", |b| {
        b.iter_batched(
            || {
                let mut f = File::open("input.txt").expect("can't open file");
                let mut buf = String::new();
                f.read_to_string(&mut buf).expect("can't read file");
                buf
            },
            |f| p1(&f),
            BatchSize::SmallInput,
        )
    });
    g.finish();
}

fn bench_p2(c: &mut Criterion) {
    let mut g = c.benchmark_group("criterion");
    g.bench_function("p2", |b| {
        b.iter_batched(
            || {
                let mut f = File::open("input.txt").expect("can't open file");
                let mut buf = String::new();
                f.read_to_string(&mut buf).expect("can't read file");
                buf
            },
            |f| p2(&f),
            BatchSize::SmallInput,
        )
    });
    g.finish()
}


fn bench_p1_swift(c: &mut Criterion) {
    let mut g = c.benchmark_group("criterion");
    g.bench_function("p1-swift", |b| {
        b.iter_batched(
            || {
                let mut f = File::open("input.txt").expect("can't open file");
                let mut buf = String::new();
                f.read_to_string(&mut buf).expect("can't read file");
                buf
            },
            |f| unsafe {day9p1ForRust(SRString::from(f.as_str()))},
            BatchSize::SmallInput,
        )
    });
    g.finish();
}

fn bench_p2_swift(c: &mut Criterion) {
    let mut g = c.benchmark_group("criterion");
    g.bench_function("p2-swift", |b| {
        b.iter_batched(
            || {
                let mut f = File::open("input.txt").expect("can't open file");
                let mut buf = String::new();
                f.read_to_string(&mut buf).expect("can't read file");
                buf
            },
            |f| unsafe {day9p2ForRust(SRString::from(f.as_str()))},
            BatchSize::SmallInput,
        )
    });
    g.finish()
}

criterion_group!(benches, bench_p1, bench_p2, bench_p1_swift, bench_p2_swift);
criterion_main!(benches);
