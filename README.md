# Rust/Swift interop with SwiftRs

Using solutions of Advent of Code 2023 day 9, this rep gives an example of
how to use SwiftRs to use Swift packages with Rust.

Benchmarks are discouraging for Swift though...

## Caveats

The swift part is included  on macOS only, as SwiftRs is not available on Linux.

## Building the Swift package of rust functions
*This is on macOS only*

As a pre-requisite you need the swift-bridge-cli commmand for cargo. This is installed by
```
cargo install swift-bridge-cli
```
To build the package *DayRust9*, do this

```
cargo clean
cargo build --release
swift-bridge-cli create-package --bridges-dir day9_rust/generated --name Day9Rust --out-dir Day9Rust --macos $(find target/release -name "libday9_rust-*.a")
```
Now the Swift package in day9_rust can be build and tests run on the swift and rust functions by
```
cd day9_swift
swift build -c release
swift test
```