use swift_rs::{swift, SRString};

swift!(pub fn day9p1_rust_bridge(s: SRString) -> i64);

swift!(pub fn day9p2_rust_bridge(s: SRString) -> i64);

extern "C" {
    pub fn day9p1_rust_bridge2(s: * mut u8, len: i64);
}
