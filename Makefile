all: Day9Rust

Day9Rust: day9_rust/src/lib.rs day9_rust/src/bridge.rs 
	cargo build -p day9_rust --release
	swift-bridge-cli create-package --bridges-dir day9_rust/generated --name Day9Rust --out-dir Day9Rust --macos target/release/libday9_rust.a
