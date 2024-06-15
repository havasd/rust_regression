use super::*;

#[swift_bridge::bridge]
pub mod bridge {
    extern "Rust" {
        pub fn p1(input: &str) -> i64;
        pub fn p2(input: &str) -> i64;
    }
}