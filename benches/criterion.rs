use criterion::{criterion_group, criterion_main, BatchSize, Criterion};

use day9_rust::{p1, p2};
use std::fs::File;
use std::io::Read;

fn bench_p1(c: &mut Criterion) {
    let mut g = c.benchmark_group("criterion");
    g.bench_function("p1-rust", |b| {
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
    g.bench_function("p2-rust", |b| {
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
#[cfg(target_os = "macos")]
use swift_rs::{self, SRString};

#[cfg(target_os = "macos")]
use day9::swift::{
    day9p1_rust_bridge, day9p2_rust_bridge,
};

#[cfg(target_os = "macos")]
fn bench_p1_swift(c: &mut Criterion) {
    use std::time::Duration;

    let mut g = c.benchmark_group("criterion");
    g.measurement_time(Duration::from_secs(7));
    g.bench_function("p1-swift", |b| {
        b.iter_batched(
            || {
                let mut f = File::open("input.txt").expect("can't open file");
                let mut buf = String::new();
                f.read_to_string(&mut buf).expect("can't read file");
                buf
            },
            |mut f| unsafe { day9p1_rust_bridge(f.as_mut_ptr(), f.len() as i64) },
            BatchSize::SmallInput,
        )
    });
    g.finish();
}

#[cfg(target_os = "macos")]
fn bench_p2_swift(c: &mut Criterion) {
    use std::time::Duration;

    let mut g = c.benchmark_group("criterion");
    g.measurement_time(Duration::from_secs(7));
    g.bench_function("p2-swift", |b| {
        b.iter_batched(
            || {
                let mut f = File::open("input.txt").expect("can't open file");
                let mut buf = String::new();
                f.read_to_string(&mut buf).expect("can't read file");
                SRString::from(buf.as_str())
            },
            |f| unsafe { day9p2_rust_bridge(&f) },
            BatchSize::SmallInput,
        )
    });
    g.finish()
}

#[cfg(target_os = "macos")]
criterion_group!(
    benches,
    bench_p1,
    bench_p2,
    bench_p1_swift,
    bench_p2_swift,
);

#[cfg(not(target_os = "macos"))]
criterion_group!(
    benches,
    bench_p1,
    bench_p2
);

criterion_main!(benches);
