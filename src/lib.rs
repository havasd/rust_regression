#[cfg(target_os = "macos")]
pub mod swift {

    extern "C" {
        pub fn day9p1_rust_bridge(s: *const u8, len: i64) -> i64;
        pub fn day9p2_rust_bridge(s: *const u8, len: i64) -> i64;

    }
}
