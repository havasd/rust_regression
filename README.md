# Swift performance abysmal ?

This branch contains solutions for the Advent of Code 2023 puzzle of day 9 for both Rust and Swift. On this branch the solution are not linked together by SwiftRs and Swift-Bridge to simplify building and running the benchmarks.

Benchmarks are given for Rust with Criterion and Divan. Run them by

```bash
cargo bench
```

and for Swift with XCTest. Run them by

```
cd day9_swift
swift test -c release
```

The results show that Swift is about 1000 times slower than Rust here, and I cannot get why (as runtimes do not differ really for both parts of the puzzle, only those for part1 are shown here):

| Language | Criterion | Divan | XCTest |
|----------|-----------|-------|--------|
| Rust Part1    | 150 µs    | 110 µs|        |
| Swift Part1   |           |       | 2 **ms**  |

Why is the Swift code so slow and how can it be improved?