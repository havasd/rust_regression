use std::{fs::File, io::Read};

use day9_rust::p1;
use day9_rust::p2;
#[cfg(target_os = "macos")]
use day9::swift::{day9p1_rust_bridge, day9p2_rust_bridge};
#[cfg(target_os = "macos")]
use swift_rs::{self, SRString};

fn main() {
    let mut f = File::open("input.txt").expect("can't open file");
    let mut buf = String::new();
    f.read_to_string(&mut buf).expect("can't read file");
    let result = p1(&buf);
    println!("{result}");
    let result = p2(&buf);
    println!("{result}");
    if cfg!(target_os = "macos") {
        let result = unsafe { day9p1_rust_bridge(SRString::from(buf.as_str())) };
        println!("{result}");
        let result = unsafe { day9p2_rust_bridge(SRString::from(buf.as_str())) };
        println!("{result}");
    }
}
