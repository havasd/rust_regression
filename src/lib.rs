#[cfg(target_os = "macos")]
pub mod swift {
    use swift_rs::{swift, SRString};

    swift!(pub fn day9p2_rust_bridge(s: &SRString) -> i64);

    extern "C" {
        pub fn day9p1_rust_bridge(s: *const u8, len: i64) -> i64;
    }
}
