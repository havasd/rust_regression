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

#[cfg(target_os = "macos")]
mod swift {
    use day9::swift::{day9p1_rust_bridge, day9p2_rust_bridge};
    use swift_rs::{self, SRString};
    #[divan::bench]
    fn part1() {
        let input: &str = divan::black_box(include_str!(
            "../input.txt",
        ));
        unsafe {
            day9p1_rust_bridge(input.as_ptr(), input.len() as i64);
        }
    }

    #[divan::bench]
    fn part2() {
        unsafe {
            day9p2_rust_bridge(&divan::black_box(SRString::from(include_str!(
                "../input.txt",
            ))));
        }
    }
}
