fn main() {
    // Run registered benchmarks.
    divan::main();
}

mod rust {
    use day9_rust::{p1, p2};

    #[divan::bench]
    fn part1() {
        p1(divan::black_box(include_str!("../input.txt",)));
    }

    #[divan::bench]
    fn part2() {
        p2(divan::black_box(include_str!("../input.txt",)));
    }
}

